#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{
	decl_module,
	decl_storage,
	decl_event,
	decl_error,
	ensure,
	StorageMap,
	// dispatch
};
use frame_system::ensure_signed;
use sp_std::prelude::*;
use sp_std::default::Default;
use codec::{
	Decode,
	Encode
};
use sp_runtime::traits::StaticLookup;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// 问题，数据存进去了，通过前端来查询发现结果是这个，无论存了什么key进去，返回都是这个
/// 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0800000001
/// 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d5000000001
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Collections<A, B> {
	owner: A,
	block_number: B,
	read_only: bool,
	some_thing: Vec<u8>,
	count: i32,
}

// type CollectionOf<T> = Collections<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber>;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	trait Store for Module<T: Config> as PoeModule {
        /// The storage item for our proofs.
        /// 它将证明映射到提出声明的用户以及声明的时间。
        /// Vec<u8是key>
        // Proofs: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
        // Proofs: map hasher(blake2_128_concat) Vec<u8> => CollectionOf<T>;
        Proofs: map hasher(blake2_128_concat) Vec<u8> => Collections<T::AccountId, T::BlockNumber>;
    }
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
        /// Event emitted when a proof has been claimed. [owner, claim, read_only, count]
        CollectionStatus(AccountId, Vec<u8>, bool, i32),
        /// Event emitted when a claim is revoked by the owner. [owner, claim]
        CollectionRevoked(AccountId, Vec<u8>),
        /// Event collection transfer [from, to]
        CollectionTransfer(AccountId, AccountId),
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Config> {
        /// The proof has already been claimed.
        ProofAlreadyClaimed,
        /// 该证明不存在，因此它不能被撤销
        NoSuchProof,
        /// 该证明已经被另一个账号声明，因此它不能被撤销
        NotProofOwner,
        ReadOnly,
        ReadStatusNoChange,
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// 允许用户队未声明的证明拥有所有权
        #[weight = 10_000]
        fn create_collection(origin, proof: Vec<u8>, some_thing: Vec<u8>,read_only: bool) {
            // 检查 extrinsic 是否签名并获得签名者
            // 如果 extrinsic 未签名，此函数将返回一个错误。
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;

            // 校验指定的证明是否被声明
            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            // 从 FRAME 系统模块中获取区块号.
            let current_block = <frame_system::Module<T>>::block_number();

            // 存储证明：发送人与区块号
            // Proofs::<T>::insert(&proof, (&sender, current_block));
			let count = 1;

            let collection = Collections{
            	owner: sender.clone(),
            	block_number: current_block,
            	some_thing: some_thing,
				read_only: read_only,
				count: count
            };

            Proofs::<T>::insert(&proof, collection);

            // 声明创建后，发送事件
            Self::deposit_event(RawEvent::CollectionStatus(sender, proof, read_only, count));
        }

        /// 允许证明所有者撤回声明
        #[weight = 10_000]
        fn revoke_collection(origin, proof: Vec<u8>) {
            //  检查 extrinsic 是否签名并获得签名者
            // 如果 extrinsic 未签名，此函数将返回一个错误。
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;

            // 校验指定的证明是否被声明
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            // 获取声明的所有者
            // let (owner, _) = Proofs::<T>::get(&proof);
            let colletcion = Proofs::<T>::get(&proof);

            let owner = colletcion.owner;

            // 验证当前的调用者是证声明的所有者
            ensure!(sender == owner, Error::<T>::NotProofOwner);

            // 从存储中移除声明
            Proofs::<T>::remove(&proof);

            // 声明抹掉后，发送事件
            Self::deposit_event(RawEvent::CollectionRevoked(sender, proof));
        }

		/// 设置可读
        #[weight = 10_000]
        fn get_collection_read_status(origin, proof: Vec<u8>) {
        	let _sender = ensure_signed(origin)?;

            // 校验指定的证明是否被声明
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
            let colletcion = Proofs::<T>::get(&proof);
            Self::deposit_event(RawEvent::CollectionStatus(colletcion.owner, proof, colletcion.read_only, colletcion.count));
        }

        /// 设置可读
        #[weight = 10_000]
        fn change_collection_readable(origin, proof: Vec<u8>, read_only: bool) {
        	let sender = ensure_signed(origin)?;

            // 校验指定的证明是否被声明
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            // 获取声明的所有者
            // let (owner, _) = Proofs::<T>::get(&proof);
            let colletcion = Proofs::<T>::get(&proof);

            let owner = colletcion.owner;

            // 验证当前的调用者是证声明的所有者
            ensure!(sender == owner, Error::<T>::NotProofOwner);

			// 确定状态变更了再更新
			ensure!(colletcion.read_only != read_only, Error::<T>::ReadStatusNoChange);

			// https://polkadot.js.org/apps/  用这个测试，substrate-front-end-template 有问题
			Proofs::<T>::mutate(&proof, |c| {
			    c.read_only = read_only;
			    c.count += 1
			    });
			Self::deposit_event(RawEvent::CollectionStatus(owner, proof, read_only, colletcion.count));

        }

		/// 转移collection
        #[weight = 10_000]
		fn transfer_connection(origin, proof: Vec<u8>, target: <T::Lookup as StaticLookup>::Source) {
			let sender = ensure_signed(origin)?;

            // 校验指定的证明是否被声明
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

			let target = T::Lookup::lookup(target)?;


            // 获取声明的所有者
            // let (owner, _) = Proofs::<T>::get(&proof);
            let colletcion = Proofs::<T>::get(&proof);

            let owner = colletcion.owner;

            // 验证当前的调用者是证声明的所有者
            ensure!(sender == owner, Error::<T>::NotProofOwner);

            // 确定状态不是只读的
            ensure!(!colletcion.read_only, Error::<T>::ReadOnly);

            // 更改collection owner
			Proofs::<T>::mutate(&proof, |c| {
			    c.owner = target.clone();
			    c.count += 1
			    });

			Self::deposit_event(RawEvent::CollectionTransfer(owner, target));

		}
	}
}
