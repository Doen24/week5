#![cfg_attr(not (feature="std"),no_std)]
/// A module for proof of existence
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet{
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*   
    };
    use frame_system::pallet_prelude::*
    use sp_std::vec::Vec;
    /// respondencs


    #[pallet::config] ///配置接口
    pub trait Config : frame_system::Config{
        type Event:From<Event<Self>> +  IsType<<Self as  frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);


    ///定义存储单元
    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub type Proofs<T:Config>=StorageMap<   //只有一个存储单元proofs，存储存证，类型StorageMap ，
        _,
        Blake2_128Concat,
        Vec<u8>,                              ///map的key是vec<u8>表示存证的哈希值
        (T::AccountId,T::BlockNumber)         /// key对应的value是两个元素组成的tuple，  第一个是用户id，第二个是存入存证的区块
     
    >;

    #[pallet::event]                         ///定义event的枚举类型
    #[pallet::metadata(T:AccountId="AccountId")]
    #[pallet::generate_deposite(pub(super) fn deposite_event)]
    pub enum Event<T:Config>{
        ClaimCreated(T::AccountId,Vec<u8>),
        ClaimRevoked(T::AccountId,Vec<u8>),
        ClaimTransfer(T::AccountId,Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T>{
        ProofsAlreadyExist，
        ClaimNotExist,
        NotClaimOwner,
        NoSuchProof,
    }

    #[pallet::hooks]                 /// 特殊函数定义在hooks中
    impl<T:Config> Hooks <BlockNumberFor<T>> for  Pallet<T> {}

    #[pallet::call]                 ///可调用函数的定义
    impl<T:Config> Pallet<T>{
        #[pallet:weight(0)]
        pub fn creat_claim(
            origin:OriginFor<T>, 
            claim:Vec<u8>) 
            ->DispatchResultWithPostInfo{
                let sender =ensure_signed(origin)?;                   ///校验发送方，返回accountid
                ensure!(!Proofs::<T>contains_key(&claim),Error::<T>::ProofsAlreadyExist);      ///检查目前存证记录中不存在该存证，否则返回错误
                
                Proofs::<T>::insert(
                    &claim,
                    (sender.clone(),frame_system::Pallet::<T>::block_number()));         
                   }
                Self::deposite_event(Event::ClaimCreated(sender,claim));
                Ok(().into())
        }

        #[pallet:weight(0)]
        pub  fn revoke_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>
        )->DispatchResultWithPostInfo{
            let sender =ensure_signed(origin)?;
            let (owner,_)=Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            ensure!(owner==sender,Error::<T>::NotClaimOwner);
            Proofs::<T>::remove(&claim);
            Self::deposite_event(Event::ClaimRevoked(sender,claim));
            Ok(().into())
        }



        #[pallet:weight(0)]
        pub fn transfer_claim(
            origin:OriginFor<T>,
            claim:Vec<u8>,
            receiver:T::AccountId
        )->DispatchResultWithPostInfo{
            let sender =ensure_signed(origin)?;

            ensure!(!Proofs::<T>contains_key(&proof),Error::<T>::NoSuchProof);
            let (owner,_)=Proofs::<T>::get(&proof).unwrap();
            let current_block = <frame_system::Pallet<T>::block_number();

            Proofs::<T>::mutate ( &proof,|value|{
                value.as_mut().unwrap().0=receiver.clone();
                value.as_mut().unwrap().1=current_block;
            });


            Self::deposite_event(Event::ClaimTransfer(sender,receiver,proof));
            Ok(().into())
        
        }

}
