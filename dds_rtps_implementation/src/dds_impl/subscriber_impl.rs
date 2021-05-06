use rust_dds_api::{
    dcps_psm::{
        InstanceHandle, InstanceStateKind, SampleLostStatus, SampleStateKind, StatusMask,
        ViewStateKind,
    },
    infrastructure::{
        entity::{Entity, StatusCondition},
        qos::{DataReaderQos, SubscriberQos, TopicQos},
    },
    return_type::DDSResult,
    subscription::{
        data_reader::AnyDataReader, data_reader_listener::DataReaderListener,
        subscriber_listener::SubscriberListener,
    },
};

use super::{
    data_reader_impl::DataReaderImpl, domain_participant_impl::DomainParticipantImpl,
    topic_impl::TopicImpl,
};

pub struct SubscriberImpl<'s, 'dp: 's, PSM: rust_rtps_pim::PIM> {
    parent: &'s DomainParticipantImpl<'dp, PSM>,
}

impl<'s, 'dp: 's, PSM: rust_rtps_pim::PIM> SubscriberImpl<'s,'dp,PSM> {
    pub(crate) fn new() {

    }
}

impl<'dr, 's: 'dr, 't: 'dr, 'dp: 's + 't, T: 't, PSM: rust_rtps_pim::PIM>
    rust_dds_api::subscription::subscriber::DataReaderFactory<'dr, 's, 't, 'dp, T>
    for SubscriberImpl<'s, 'dp, PSM>
{
    type TopicType = TopicImpl<'t, 'dp, T, PSM>;
    type DataReaderType = DataReaderImpl<'dr, 's, 't, 'dp, T, PSM>;

    fn create_datareader(
        &'dr self,
        _a_topic: &'dr Self::TopicType,
        _qos: Option<DataReaderQos<'dr>>,
        _a_listener: Option<&'dr (dyn DataReaderListener<DataType = T> + 'dr)>,
        _mask: StatusMask,
    ) -> Option<Self::DataReaderType> {
        todo!()
    }

    fn delete_datareader(&self, _a_datareader: &Self::DataReaderType) -> DDSResult<()> {
        todo!()
    }

    fn lookup_datareader<'a>(
        &'a self,
        _topic: &'a Self::TopicType,
    ) -> Option<Self::DataReaderType> {
        todo!()
    }
}

impl<'s, 'dp: 's, PSM: rust_rtps_pim::PIM>
    rust_dds_api::subscription::subscriber::Subscriber<'s, 'dp> for SubscriberImpl<'s, 'dp, PSM>
{
    fn begin_access(&self) -> DDSResult<()> {
        todo!()
    }

    fn end_access(&self) -> DDSResult<()> {
        todo!()
    }

    fn notify_datareaders(&self) -> DDSResult<()> {
        todo!()
    }

    fn get_sample_lost_status(&self, _status: &mut SampleLostStatus) -> DDSResult<()> {
        todo!()
    }

    fn delete_contained_entities(&self) -> DDSResult<()> {
        todo!()
    }

    fn set_default_datareader_qos(&self, _qos: Option<DataReaderQos<'s>>) -> DDSResult<()> {
        todo!()
    }

    fn get_default_datareader_qos(&self) -> DDSResult<DataReaderQos<'s>> {
        todo!()
    }

    fn copy_from_topic_qos(
        &self,
        _a_datareader_qos: &mut DataReaderQos<'s>,
        _a_topic_qos: &TopicQos,
    ) -> DDSResult<()> {
        todo!()
    }

    fn get_datareaders(
        &self,
        _readers: &mut [&mut dyn AnyDataReader],
        _sample_states: &[SampleStateKind],
        _view_states: &[ViewStateKind],
        _instance_states: &[InstanceStateKind],
    ) -> DDSResult<()> {
        todo!()
    }

    fn get_participant(
        &self,
    ) -> &dyn rust_dds_api::domain::domain_participant::DomainParticipant<'dp> {
        self.parent
    }
}

impl<'s, 'dp: 's, PSM: rust_rtps_pim::PIM> Entity for SubscriberImpl<'s, 'dp, PSM> {
    type Qos = SubscriberQos<'s>;
    type Listener = &'s (dyn SubscriberListener + 's);

    fn set_qos(&self, _qos: Option<Self::Qos>) -> DDSResult<()> {
        todo!()
        // Ok(self
        //     .impl_ref
        //     .upgrade()
        //     .ok_or(DDSError::AlreadyDeleted)?
        //     .lock()
        //     .unwrap()
        //     .set_qos(qos))
    }

    fn get_qos(&self) -> DDSResult<Self::Qos> {
        todo!()
        // Ok(self
        //     .impl_ref
        //     .upgrade()
        //     .ok_or(DDSError::AlreadyDeleted)?
        //     .lock()
        //     .unwrap()
        //     .get_qos()
        //     .clone())
    }

    fn set_listener(
        &self,
        _a_listener: Option<Self::Listener>,
        _mask: StatusMask,
    ) -> DDSResult<()> {
        todo!()
    }

    fn get_listener(&self) -> DDSResult<Option<Self::Listener>> {
        todo!()
    }

    fn get_statuscondition(&self) -> StatusCondition {
        todo!()
    }

    fn get_status_changes(&self) -> StatusMask {
        todo!()
    }

    fn enable(&self) -> DDSResult<()> {
        todo!()
    }

    fn get_instance_handle(&self) -> DDSResult<InstanceHandle> {
        todo!()
    }
}
