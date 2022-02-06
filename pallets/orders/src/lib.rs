//                              Næ§@@@ÑÉ©
//                        æ@@@@@@@@@@@@@@@@@@
//                    Ñ@@@@?.?@@@@@@@@@@@@@@@@@@@N
//                 ¶@@@@@?^%@@.=@@@@@@@@@@@@@@@@@@@@
//               N@@@@@@@?^@@@»^@@@@@@@@@@@@@@@@@@@@@@
//               @@@@@@@@?^@@@».............?@@@@@@@@@É
//              Ñ@@@@@@@@?^@@@@@@@@@@@@@@@@@@'?@@@@@@@@Ñ
//              @@@@@@@@@?^@@@»..............»@@@@@@@@@@
//              @@@@@@@@@?^@@@»^@@@@@@@@@@@@@@@@@@@@@@@@
//              @@@@@@@@@?^ë@@&.@@@@@@@@@@@@@@@@@@@@@@@@
//               @@@@@@@@?^´@@@o.%@@@@@@@@@@@@@@@@@@@@©
//                @@@@@@@?.´@@@@@ë.........*.±@@@@@@@æ
//                 @@@@@@@@?´.I@@@@@@@@@@@@@@.&@@@@@N
//                  N@@@@@@@@@@ë.*=????????=?@@@@@Ñ
//                    @@@@@@@@@@@@@@@@@@@@@@@@@@@¶
//                        É@@@@@@@@@@@@@@@@Ñ¶
//                             Næ§@@@ÑÉ©

// Copyright 2020 Chris D'Costa
// This file is part of Totem Live Accounting.
// Authors:
// - Félix Daudré-Vignier   email: felix@totemaccounting.com
// - Chris D'Costa          email: chris.dcosta@totemaccounting.com

// Totem is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Totem is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Totem.  If not, see <http://www.gnu.org/licenses/>.

//! Totem Orders Module
//!
//! # Overview
//!
//! The orders module supports creation of purchase orders and tasks and other types of market order.
//!
//! A basic workflow is as follows:
//!
//! * In general orders are assigned to a partner that the ordering identity already knows and is required to be accepted by that party to become active.
//! * Orders can be made without already knowing the seller - these are called market orders
//! * The order can be prefunded by calling into the prefunding module, which updates the accounting ledgers.
//! * Once the order is accepted, the work must begin, and once completed, the vendor sets the state to completed.
//! * The completion state also generates the invoice, and relevant accounting postings for both the buyer and the seller.
//! * The completed work is then approved by the buyer (or disputed or rejected). An approval triggers the release of prefunds and
//! the invoice is marked as settled in the accounts for both parties
//!
//! The main types used in this module are:
//!
//! * Product = Hash;
//! * UnitPrice = i128; // This does not need a unit of currency because it is allways the internal functional currency
//! * Quantity = u128;
//! * UnitOfMeasure = u16;
//! * buy_or_sell: u16, // 0: buy, 1: sell, extensible
//! * amount: AccountBalanceOf<T>, // amount should be the sum of all the items untiprices * quantities
//! * open_closed: bool, // 0: open(true) 1: closed(false)
//! * order_type: u16, // 0 Services, 1 Goods, 2 Inventory
//! * deadline: u64, // prefunding acceptance deadline
//! * due_date: u64, // due date is the future delivery date (in blocks)

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
mod pallet {

    use frame_support::{fail, pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;

    use sp_runtime::traits::Convert;
    use sp_std::{prelude::*, vec};

    use totem_common::{StorageMapExt, TryConvert};
    use totem_primitives::{
        accounting::Posting,
        bonsai::Storing,
        orders::{
            ApprovalStatus, OrderHeader, OrderItem, OrderStatus, TxKeysL, TxKeysM, Validating,
        },
        prefunding::{Encumbrance, LockStatus},
    };

    type CurrencyBalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn owner)]
    pub type Owner<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<T::Hash>>;

    #[pallet::storage]
    #[pallet::getter(fn beneficiary)]
    pub type Beneficiary<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<T::Hash>>;

    #[pallet::storage]
    #[pallet::getter(fn approver)]
    pub type Approver<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<T::Hash>>;

    #[pallet::storage]
    #[pallet::getter(fn postulate)]
    pub type Postulate<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Vec<T::Hash>>;

    #[pallet::storage]
    #[pallet::getter(fn orders)]
    pub type Orders<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, OrderHeader<T::AccountId>>;

    #[pallet::storage]
    #[pallet::getter(fn order_items)]
    pub type OrderItems<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, Vec<OrderItem<T::Hash>>>;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Accounting: Posting<
            Self::AccountId,
            Self::Hash,
            Self::BlockNumber,
            CurrencyBalanceOf<Self>,
        >;
        type Bonsai: Storing<Self::Hash>;
        type Currency: Currency<Self::AccountId>;
        type OrdersConverter: TryConvert<i128, CurrencyBalanceOf<Self>>
            + Convert<u32, Self::BlockNumber>
            + Convert<Self::BlockNumber, u32>;
        type Prefunding: Encumbrance<
            Self::AccountId,
            Self::Hash,
            Self::BlockNumber,
            CurrencyBalanceOf<Self>,
        >;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Cannot change an order that you are not the approver of.
        NotApprover,
        /// This hash already exists! Try again.
        HashExists,
        /// This hash does not exit.
        HashExists2,
        /// This hash does not exit.
        HashExists3,
        /// Cannot make an order for yourself!
        CannotBeBoth,
        /// Cannot make an order for yourself!
        CannotBeBoth2,
        /// You should not be doing this!
        URNobody,
        /// Order already accepted - cannot change now!
        OrderStatus1,
        /// Incorrect Order Status!
        OrderStatus2,
        /// The order has an unkown state!
        OrderStatus3,
        /// The submitted status not allowed.
        ApprStatus,
        /// Already approved!
        Approved,
        /// Order status is not allowed!
        StatusNotAllowed1,
        /// Order already accepted. Order status is not allowed!
        StatusNotAllowed2,
        /// The order has a status that cannot be changed!
        StatusNotAllowed3,
        /// The order has an unkown state!
        StatusNotAllowed4,
        /// The order has an unkown state!
        StatusNotAllowed5,
        /// This is not your order or wrong status.
        StatusNotAllowed6,
        /// Not allowed to fulfill your own order!
        Fulfiller,
        /// Amount cannot be less than zero!
        Amount,
        /// Deadline is too short! 48 hours is minimum deadline.
        ShortDeadline,
        /// Due date must be at least 1 hour after deadline.
        ShortDueDate,
        /// This situation is not implemented yet: Invoice is disputed.
        NotImplmented1,
        /// Unable to fetch order with this reference.
        GettingOrder,
        /// Error setting prefunding state.
        SetPrefundState,
        /// Error from prefunding module - in check approver.
        InPrefunding1,
        /// Error in Processing Order Acceptance status.
        InPrefunding2,
        /// Error in rejecting order adjusting commander settings.
        InPrefunding3,
        /// Error in rejecting order releasing commander lock.
        InPrefunding4,
        /// Error in prefunding module to send invoice.
        InPrefunding5,
        /// Error in prefunding settling invoice.
        InPrefunding6,
        /// Error setting the first prefunding request.
        InPrefunding7,
        /// Cannot make an market order against a parent order.
        MarketOrder,
        /// The amount is invalid and cannot be handled safely.
        AmountOverflow,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Only the owner of an order can delete it provided no work has been done on it.
        #[pallet::weight(0/*TODO*/)]
        pub fn delete_order(
            origin: OriginFor<T>,
            tx_keys_medium: TxKeysM<T::Hash>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            <T::Bonsai as Storing<T::Hash>>::start_tx(tx_keys_medium.tx_uid)?;

            // Only delete order if it has not been accepted by the fulfiller.
            match Self::orders(&tx_keys_medium.record_id) {
                Some(order) => {
                    // Order is owned by sender, status unaccepted a
                    let approver: T::AccountId = order.approver;
                    let order_status: u16 = order.order_status;
                    if (&approver, order_status) == (&who, 0_u16) {
                        Owner::<T>::mutate_or_err(&order.commander, |owner| {
                            owner.retain(|v| v != &tx_keys_medium.record_id)
                        })?;
                        Beneficiary::<T>::mutate_or_err(&order.fulfiller, |owner| {
                            owner.retain(|v| v != &tx_keys_medium.record_id)
                        })?;
                        // <Approver<T>>::mutate(&approver, |owner| {
                        Approver::<T>::mutate_or_err(approver, |owner| {
                            owner.retain(|v| v != &tx_keys_medium.record_id)
                        })?;
                        Postulate::<T>::remove(&tx_keys_medium.record_id);
                        Orders::<T>::remove(&tx_keys_medium.record_id);
                        OrderItems::<T>::remove(&tx_keys_medium.record_id);
                    } else {
                        fail!(Error::<T>::StatusNotAllowed6);
                    }
                }
                // Order does not exist
                None => fail!(Error::<T>::HashExists3),
            }
            <T::Bonsai as Storing<T::Hash>>::end_tx(tx_keys_medium.tx_uid)?;

            Ok(().into())
        }

        /// Creates either a sales order or a purchase order with multi-line items and a parent order.
        /// Will be used for the marketplace in order to set up open orders.
        #[pallet::weight(0/*TODO*/)]
        pub fn create_order(
            origin: OriginFor<T>,
            approver: T::AccountId,
            fulfiller: T::AccountId,
            buy_or_sell: u16,
            total_amount: i128,
            market_order: bool,
            order_type: u16,
            deadline: u32,
            due_date: u32,
            order_items: Vec<OrderItem<T::Hash>>,
            tx_keys_large: TxKeysL<T::Hash>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            <T::Bonsai as Storing<T::Hash>>::start_tx(tx_keys_large.tx_uid)?;
            // Check that the supplied record_id does not exist
            if Orders::<T>::contains_key(&tx_keys_large.record_id) {
                fail!(Error::<T>::HashExists);
            }

            let mut approval_status = ApprovalStatus::Submitted;
            // Check that it is an open order
            if market_order {
                // process open order - ignore fulfiller
                // check that the order does not have a parent - by default the parent and the record_id must be the same
                if tx_keys_large.record_id == tx_keys_large.parent_id {
                } else {
                    fail!(Error::<T>::MarketOrder);
                }
                // Go further - Store the Order
                ();
            } else {
                // closed order, fulfiller must be completed and it must not be the origin
                if fulfiller == who {
                    fail!(Error::<T>::CannotBeBoth2);
                }
                // The order may have a parent - by default the parent and the record_id are the same, but they may also be different
                if tx_keys_large.record_id == tx_keys_large.parent_id {
                    // This order has no parent therefore is a simple unfunded order with a known fulfiller
                    // TODO
                    ();
                } else {
                    // This order has a parent therefore it is a proposal and this means there is a fulfiller
                    // check that that the parent hash exists
                    if Orders::<T>::contains_key(&tx_keys_large.parent_id) == false {
                        fail!(Error::<T>::HashExists2);
                    };
                    // if the approver is also the initiator of the order then automatically approve the order
                    if Self::check_approver(who.clone(), approver, tx_keys_large.record_id) {
                        // the order is approved because the approver is the commander.
                        approval_status = ApprovalStatus::Accepted;
                    } else {
                        // the order is not yet approved.
                        // This is NOT an error but requires further processing by the approver.
                        // As this is a proposal against a parent order then associate the child with the parent
                        // This does not happen when it is a simple order
                        Postulate::<T>::mutate_or_err(&tx_keys_large.parent_id, |v| {
                            v.push(tx_keys_large.record_id)
                        })?;
                        // <TxList<T>>::mutate(list_key, |tx_list| tx_list.push(u));
                    }
                }
                let order_header: OrderHeader<T::AccountId> = OrderHeader {
                    commander: who.clone(),
                    fulfiller: fulfiller.clone(),
                    approver: who.clone(),
                    order_status: 0u16,
                    approval_status,
                    buy_or_sell,
                    amount: total_amount,
                    market_order,
                    order_type,
                    deadline,
                    due_date,
                };
                Self::set_order(
                    who,
                    fulfiller,
                    tx_keys_large.record_id,
                    order_header,
                    order_items,
                )?;
            }
            <T::Bonsai as Storing<T::Hash>>::end_tx(tx_keys_large.tx_uid)?;
            Self::deposit_event(Event::OrderCreated(
                tx_keys_large.tx_uid,
                tx_keys_large.record_id,
            ));

            Ok(().into())
        }

        /// Create Simple Prefunded Service Order.
        /// Can specify an approver. If the approver is the same as the sender then the order is considered approved by default.
        #[pallet::weight(0/*TODO*/)]
        pub fn create_spfso(
            origin: OriginFor<T>,
            approver: T::AccountId,
            fulfiller: T::AccountId,
            buy_or_sell: u16,               // 0: buy, 1: sell, extensible
            total_amount: i128, // amount should be the sum of all the items untiprices * quantities
            market_order: bool, // 0: open(false) 1: closed(true)
            order_type: u16,    // 0: service, 1: inventory, 2: asset extensible
            deadline: u32,      // prefunding acceptance deadline
            due_date: u32,      // due date is the future delivery date (in blocks)
            order_item: OrderItem<T::Hash>, // for simple items there will only be one item, item number is accessed by its position in Vec
            bonsai_token: T::Hash,          // Bonsai data Hash
            tx_uid: T::Hash,                // Bonsai data Hash
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            <T::Bonsai as Storing<T::Hash>>::start_tx(tx_uid)?;
            // Generate Hash for order
            let order_hash =
                <T as Config>::Accounting::get_pseudo_random_hash(who.clone(), approver.clone());
            if Orders::<T>::contains_key(&order_hash) {
                fail!(Error::<T>::HashExists);
            }
            Self::set_simple_prefunded_service_order(
                who,
                approver,
                fulfiller,
                buy_or_sell,
                total_amount,
                market_order,
                order_type,
                deadline,
                due_date,
                order_hash,
                order_item,
                bonsai_token,
                tx_uid,
            )?;
            <T::Bonsai as Storing<T::Hash>>::end_tx(tx_uid)?;

            Self::deposit_event(Event::OrderCreated(tx_uid, order_hash));

            Ok(().into())
        }

        /// Change Simple Prefunded Service Order.
        /// Can only be changed by the original ordering party,
        /// and only before it is accepted and the deadline or due date is not passed.
        #[pallet::weight(0/*TODO*/)]
        pub fn change_spfso(
            origin: OriginFor<T>,
            approver: T::AccountId,
            fulfiller: T::AccountId,
            amount: i128,
            deadline: u32,
            due_date: u32,
            order_item: OrderItem<T::Hash>,
            record_id: T::Hash,
            bonsai_token: T::Hash,
            tx_uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            // check owner of this record
            <T::Bonsai as Storing<T::Hash>>::start_tx(tx_uid)?;
            Self::change_simple_prefunded_order(
                who,
                approver,
                fulfiller,
                amount,
                deadline,
                due_date,
                order_item,
                record_id,
                bonsai_token,
            )?;
            <T::Bonsai as Storing<T::Hash>>::end_tx(tx_uid)?;

            Self::deposit_event(Event::OrderUpdated(tx_uid));

            Ok(().into())
        }

        /// Sets the approval status of an order.
        /// Can only be used by the nominated approver (must be known to the ordering party).
        #[pallet::weight(0/*TODO*/)]
        pub fn change_approval(
            origin: OriginFor<T>,
            h: T::Hash,
            s: ApprovalStatus,
            b: T::Hash,
            tx_uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            <T::Bonsai as Storing<T::Hash>>::start_tx(tx_uid)?;
            Self::change_approval_state(who, h, s, b)?;
            <T::Bonsai as Storing<T::Hash>>::end_tx(tx_uid)?;
            Self::deposit_event(Event::InvoiceSettled(h));

            Ok(().into())
        }

        /// Can be used by buyer or seller.
        /// Buyer - Used by the buyer to accept or reject (TODO) the invoice that was raised by the seller.
        /// Seller - Used to accept, reject or invoice the order.
        #[pallet::weight(0/*TODO*/)]
        pub fn handle_spfso(
            origin: OriginFor<T>,
            h: T::Hash,
            s: OrderStatus,
            tx_uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            <T::Bonsai as Storing<T::Hash>>::start_tx(tx_uid)?;
            // get order details and determine if the sender is the buyer or the seller
            let order_hdr = Self::orders(&h).ok_or(Error::<T>::GettingOrder)?;
            let commander = order_hdr.commander.clone();
            let fulfiller = order_hdr.fulfiller.clone();
            if who == commander {
                // This is the buyer
                //TODO if the order us passed as an arg it doesn't need to be read again
                Self::accept_prefunded_invoice(who, h, s, order_hdr, tx_uid)?;
                Self::deposit_event(Event::InvoiceSettled(tx_uid));
            } else if who == fulfiller {
                // This is the seller
                //TODO if the order us passed as an arg it doesn't need to be read again
                if let Err(_) =
                    Self::set_state_simple_prefunded_closed_order(who, h, s, order_hdr, tx_uid)
                {
                    fail!(Error::<T>::SetPrefundState);
                }
            } else {
                fail!(Error::<T>::URNobody)
            }

            <T::Bonsai as Storing<T::Hash>>::end_tx(tx_uid)?;

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        OrderCreated(T::Hash, T::Hash),
        OrderUpdated(T::Hash),
        OrderCreatedForApproval(T::Hash),
        OrderCreatedForApproval2(T::Hash),
        OrderStatusUpdate(T::Hash),
        OrderCompleted(T::Hash),
        InvoiceSettled(T::Hash),
    }

    impl<T: Config> Pallet<T> {
        /// Create Open Order.
        /// This function simply stores an open sales or purchase order. It is intended for the marketplace,
        /// yet it can be a complex purchase or sales order.
        ///
        /// The order can have another party as an approver or not
        /// * The order is not underwritten by prefunding
        /// * Because this is creation it cannot have a parent
        ///
        /// The approver should be able to set the status, and once approved the process should continue further
        /// pending_approval (0), approved(1), rejected(2) are the tree states to be set
        /// If the status is 2 the commander may edit and resubmit
        fn check_approver(c: T::AccountId, a: T::AccountId, h: T::Hash) -> bool {
            // If the approver is the same as the commander then it is approved by default & update accordingly
            // If the approver is not the commander, then update but also set the status to pending approval.
            // You should gracefully exit after this function call in this case.
            let approved = c == a;

            let _ = Approver::<T>::mutate_or_err(&a, |approver| approver.push(h));

            approved
        }

        /// API Open an order for a specific AccountId and prefund it. This is equivalent to an encumbrance.
        /// The amount is the functional currency and conversions are not necessary at this stage of accounting.
        /// The UI therefore handles presentation or reporting currency translations at spot rate.
        /// This is not for goods.
        /// If the order is open, the the fulfiller is ignored.
        /// Order type is generally goods (0) or services (1) but is left open for future-proofing.
        fn set_simple_prefunded_service_order(
            commander: T::AccountId,
            approver: T::AccountId,
            fulfiller: T::AccountId,
            buy_or_sell: u16,   // 0: buy, 1: sell, extensible
            amount: i128,       // amount should be the sum of all the items untiprices * quantities
            market_order: bool, // 0: open(false) 1: closed(true)
            order_type: u16,    // 0: personal, 1: business, extensible
            deadline: u32,      // prefunding acceptance deadline
            due_date: u32,      // due date is the future delivery date (in blocks)
            order_hash: T::Hash,
            order_item: OrderItem<T::Hash>, // for simple items there will only be one item, item number is accessed by its position in Vec
            bonsai_token: T::Hash,
            uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            // Set order status to submitted by default
            // submitted(0), accepted(1), rejected(2), disputed(3), blocked(4), invoiced(5),
            let order_status: OrderStatus = 0;

            // TODO Rewrite this MARKET_ORDER reversing the bool. This is because the API open_closed will be replaced by market_order bool.
            // let mut market_order: bool = false;
            let fulfiller_override = if market_order {
                commander.clone()
            } else {
                if commander == fulfiller {
                    fail!(Error::<T>::CannotBeBoth);
                }
                fulfiller.clone()
            };

            // check or set the approver status
            if Self::check_approver(commander.clone(), approver.clone(), order_hash) {
                // the order is approved.
                let approval_status = ApprovalStatus::Accepted;
                let deadline_converted = T::OrdersConverter::convert(deadline);
                // approval status has been set to approved, continue.
                // Set prefunding first. It does not matter if later the process fails, as this is locking funds for the commander
                // The risk is that they cannot get back the funds until after the deadline, even of they want to cancel.
                let balance_amount =
                    T::OrdersConverter::try_convert(amount).ok_or(Error::<T>::AmountOverflow)?;

                if let Err(_) = Self::set_prefunding(
                    commander.clone(),
                    fulfiller.clone(),
                    balance_amount,
                    deadline_converted,
                    order_hash,
                    uid,
                ) {
                    // Error from setting prefunding "somewhere" ;)
                    fail!(Error::<T>::InPrefunding1);
                }
                let order_header: OrderHeader<T::AccountId> = OrderHeader {
                    commander: commander.clone(),
                    fulfiller: fulfiller_override,
                    approver,
                    order_status,
                    approval_status,
                    buy_or_sell,
                    amount,
                    market_order,
                    order_type,
                    deadline,
                    due_date,
                };
                let vec_order_items = vec![order_item];
                Self::set_order(
                    commander,
                    fulfiller,
                    order_hash,
                    order_header,
                    vec_order_items,
                )?;
            } else {
                // the order is not yet approved.
                // This is NOT an error but requires further processing by the approver. Exiting gracefully.
                Self::deposit_event(Event::OrderCreatedForApproval(uid));
            }

            // claim hash in Bonsai
            <T::Bonsai as Storing<T::Hash>>::claim_data(order_hash, bonsai_token)?;

            Ok(().into())
        }

        /// Calls the prefunding module to lock funds. This does not perform an update or lock release.
        fn set_prefunding(
            c: T::AccountId,
            f: T::AccountId,
            a: CurrencyBalanceOf<T>,
            d: T::BlockNumber,
            o: T::Hash,
            u: T::Hash,
        ) -> DispatchResultWithPostInfo {
            if let Err(_) = T::Prefunding::prefunding_for(c, f, a, d, o, u) {
                fail!(Error::<T>::InPrefunding7);
            }

            Ok(().into())
        }

        /// Stores the order data and sets the order status.
        fn set_order(
            c: T::AccountId,
            f: T::AccountId,
            o: T::Hash,
            h: OrderHeader<T::AccountId>,
            i: Vec<OrderItem<T::Hash>>,
        ) -> DispatchResultWithPostInfo {
            // Set hash for commander
            Owner::<T>::mutate_or_err(&c, |owner| owner.push(o.clone()))?;
            // This will be a market order if the fulfiller is the same as the commander
            // In this case do not set the beneficiary storage
            if c != f {
                // Set hash for fulfiller
                Beneficiary::<T>::mutate_or_err(&f, |beneficiary| beneficiary.push(o.clone()))?;
            }
            // Set details of Order
            Orders::<T>::insert(&o, h);
            OrderItems::<T>::insert(&o, i);

            Ok(().into())
        }

        /// API This function is used to accept or reject the order by the named approver. Mainly used for the API.
        fn change_approval_state(
            a: T::AccountId,
            h: T::Hash,
            s: ApprovalStatus,
            b: T::Hash,
        ) -> DispatchResultWithPostInfo {
            // is the supplied account the approver of the hash supplied?
            let mut order_hdr: OrderHeader<T::AccountId> = Self::orders(&h).ok_or("some error")?;
            if a == order_hdr.approver && order_hdr.order_status == 0 {
                match order_hdr.order_status {
                    0 | 2 => {
                        // can only change to approved (1)
                        match s {
                            ApprovalStatus::Accepted => (),
                            _ => fail!(Error::<T>::ApprStatus),
                        }
                    }
                    1 => {
                        // Can only change to 0 or 2
                        match s {
                            ApprovalStatus::Submitted | ApprovalStatus::Rejected => (),
                            _ => fail!(Error::<T>::ApprStatus),
                        }
                    }
                    _ => fail!(Error::<T>::ApprStatus),
                }
                // All tests passed, set status to whatever.
                order_hdr.order_status = s as u16;
                Orders::<T>::insert(&h, order_hdr);
            } else {
                fail!(Error::<T>::NotApprover);
            }

            Self::deposit_event(Event::OrderStatusUpdate(b));

            Ok(().into())
        }

        /// API Allows commander to change the order either before it is accepted by beneficiary, or
        /// when it has been rejected by approver.
        fn change_simple_prefunded_order(
            commander: T::AccountId,
            approver: T::AccountId,
            fulfiller: T::AccountId,
            amount: i128,
            deadline: u32,
            due_date: u32,
            order_item: OrderItem<T::Hash>,
            reference: T::Hash,
            bonsai_token: T::Hash,
        ) -> DispatchResultWithPostInfo {
            // Check that the hash exist
            // let order_hdr: OrderHeader<T::AccountId> = Self:order_header(&reference).ok_or("some error")?;
            let order_hdr: OrderHeader<T::AccountId> =
                Self::orders(&reference).ok_or("some error")?;
            // check that the Order state is 0 or 2 (submitted or rejected)
            // check that the approval is 0 or 2 pending approval or rejected
            match order_hdr.order_status {
                0 | 2 => {
                    match order_hdr.approval_status {
                        ApprovalStatus::Submitted | ApprovalStatus::Rejected => (), // submitted pending approval or rejected
                        ApprovalStatus::Accepted => fail!(Error::<T>::Approved),
                    }
                }
                1 => fail!(Error::<T>::OrderStatus1),
                _ => fail!(Error::<T>::OrderStatus2),
            };
            // check that at least one of these has changed:
            // let mut dl: u64;
            // let mut dd: u64;
            let current_block = frame_system::Pallet::<T>::block_number();
            // apply a new fulfiller but check that it isn't the commander
            if order_hdr.commander == commander {
                fail!(Error::<T>::Fulfiller);
            }
            if order_hdr.amount != amount {
                if amount < 0i128 {
                    fail!(Error::<T>::Amount);
                }

                // IMPORTANT TODO
                // Check that the amount is the sum of all the items
            }

            let current_block_converted: u32 = T::OrdersConverter::convert(current_block);
            if order_hdr.deadline != deadline {
                // TODO This may be unusable/unworkable needs trying out
                // 48 hours is the minimum deadline
                // every time there is a change the deadline gets pushed back by 48 hours byond the current block
                let min_deadline = current_block_converted + 11520_u32;
                if deadline < min_deadline {
                    fail!(Error::<T>::ShortDeadline);
                }
                // dl = deadline;
            }
            if order_hdr.due_date != due_date {
                // due date must be at least 1 hours after deadline (TODO - Validate! as this is a guess)
                // This is basically adding 49 hours to the current block
                let minimum_due_date = current_block_converted + 11760_u32;
                if due_date < minimum_due_date {
                    fail!(Error::<T>::ShortDueDate);
                }
                // dd = due_date;
            }
            // Create Order sub header
            let order_header: OrderHeader<T::AccountId> = OrderHeader {
                commander,
                fulfiller: fulfiller.clone(),
                approver,
                order_status: 0,
                approval_status: order_hdr.approval_status,
                buy_or_sell: order_hdr.buy_or_sell,
                amount,
                market_order: order_hdr.market_order,
                order_type: order_hdr.order_type,
                deadline,
                due_date,
            };
            // currently just places all the items in the storage WITHOUT CHECKING
            // TODO check for changes and confirm that amount = sum of all amounts
            let vec_order_items = vec![order_item];

            Self::set_order(
                order_hdr.commander,
                fulfiller,
                reference,
                order_header,
                vec_order_items,
            )?;
            // prefunding can only be cancelled if deadline has passed, otherwise the prefunding remains as a deposit
            // TODO we could use the cancel prefunding function to do this.
            // change hash in Bonsai
            <T::Bonsai as Storing<T::Hash>>::claim_data(reference, bonsai_token)?;

            Ok(().into())
        }

        /// Used by the beneficiary (fulfiller) to accept, reject or invoice the order.
        /// It effectively creates a state change for the order and the prefunding.
        /// When accepting, the order is locked for the beneficiary or when rejected the funds are released for the order owner.
        fn set_state_simple_prefunded_closed_order(
            f: T::AccountId,
            h: T::Hash,
            order_status: OrderStatus,
            order: OrderHeader<T::AccountId>,
            uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            match order.order_status {
                0 => {
                    // Order not accepted yet. Update the status in this module
                    match order_status {
                        1 => {
                            // Order Accepted
                            // Update the prefunding status (confirm locked funds)
                            let lock = LockStatus::Locked;
                            if let Err(_e) = T::Prefunding::set_release_state(f, lock, h, uid) {
                                fail!(Error::<T>::InPrefunding2)
                            }
                        }
                        2 => {
                            // order rejected
                            let lock = LockStatus::Unlocked;
                            // We do not need to set release state for releasing funds for fulfiller.
                            // set release state for releasing funds for commander.
                            if let Err(_e) = T::Prefunding::set_release_state(
                                order.commander.clone(),
                                lock,
                                h,
                                uid,
                            ) {
                                fail!(Error::<T>::InPrefunding3)
                            }
                            // now release the funds lock
                            if let Err(_e) = T::Prefunding::unlock_funds_for_owner(
                                order.commander.clone(),
                                h,
                                uid,
                            ) {
                                fail!(Error::<T>::InPrefunding4)
                            }
                        }
                        _ => fail!(Error::<T>::StatusNotAllowed1),
                    }
                }
                // Order already in accepted state - Update the status
                1 => {
                    match order_status {
                        5 => {
                            // Order Completed. Now we are going to issue the invoice.
                            if let Err(_e) = T::Prefunding::send_simple_invoice(
                                f,
                                order.commander.clone(),
                                order.amount,
                                h,
                                uid,
                            ) {
                                fail!(Error::<T>::InPrefunding5)
                            }
                        }
                        _ => fail!(Error::<T>::StatusNotAllowed2),
                    }
                }
                2 | 5 => fail!(Error::<T>::StatusNotAllowed3),
                _ => fail!(Error::<T>::StatusNotAllowed4),
            }

            Orders::<T>::insert(
                &h,
                OrderHeader {
                    order_status,
                    ..order
                },
            );

            Self::deposit_event(Event::OrderCompleted(uid));

            Ok(().into())
        }

        /// Used by the buyer to accept or reject (TODO) the invoice that was raised by the seller.
        fn accept_prefunded_invoice(
            o: T::AccountId,
            h: T::Hash,
            order_status: OrderStatus,
            order: OrderHeader<T::AccountId>,
            uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            // check that this is the fulfiller
            if order.order_status != 5 {
                fail!(Error::<T>::OrderStatus3)
            }

            // Order has been invoiced. The buyer is now deciding to accept or other
            match order_status {
                // Invoice is disputed. TODO provide the ability to change the invoice and resubmit
                3 => fail!(Error::<T>::NotImplmented1),
                6 => {
                    // Invoice Accepted. Now pay-up!.
                    if let Err(_e) = T::Prefunding::settle_prefunded_invoice(o, h, uid) {
                        fail!(Error::<T>::InPrefunding6)
                    }
                    Self::deposit_event(Event::InvoiceSettled(uid));
                }
                _ => fail!(Error::<T>::StatusNotAllowed5),
            }
            // Update the status in this module

            Orders::<T>::insert(
                &h,
                OrderHeader {
                    order_status,
                    ..order
                },
            );

            Ok(().into())
        }

        #[allow(dead_code)/*TODO use it */]
        /// This is used by any party that wants to accept a market order in whole or part.
        /// This is non-blocking and can accept many applicants
        fn postulate_simple_prefunded_open_order() -> DispatchResultWithPostInfo {
            fail!("TODO")
        }
    }

    impl<T: Config> Validating<T::AccountId, T::Hash> for Pallet<T> {
        /// Check that the order is somehow managed by this identity. Mainly used for BONSAI.
        fn is_order_party(o: T::AccountId, r: T::Hash) -> bool {
            Self::orders(r)
                .map(|order| o == order.commander || o == order.fulfiller || o == order.approver)
                .unwrap_or(false)
        }
    }
}
