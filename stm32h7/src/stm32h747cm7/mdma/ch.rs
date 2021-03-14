#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](ifcr) module"]
pub type IFCR = crate::Reg<u32, _IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFCR;
#[doc = "`write(|w| ..)` method takes [ifcr::W](ifcr::W) writer structure"]
impl crate::Writable for IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr](esr) module"]
pub type ESR = crate::Reg<u32, _ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESR;
#[doc = "`read()` method returns [esr::R](esr::R) reader structure"]
impl crate::Readable for ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](tcr) module"]
pub type TCR = crate::Reg<u32, _TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR;
#[doc = "`read()` method returns [tcr::R](tcr::R) reader structure"]
impl crate::Readable for TCR {}
#[doc = "`write(|w| ..)` method takes [tcr::W](tcr::W) writer structure"]
impl crate::Writable for TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bndtr](bndtr) module"]
pub type BNDTR = crate::Reg<u32, _BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BNDTR;
#[doc = "`read()` method returns [bndtr::R](bndtr::R) reader structure"]
impl crate::Readable for BNDTR {}
#[doc = "`write(|w| ..)` method takes [bndtr::W](bndtr::W) writer structure"]
impl crate::Writable for BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar](sar) module"]
pub type SAR = crate::Reg<u32, _SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR;
#[doc = "`read()` method returns [sar::R](sar::R) reader structure"]
impl crate::Readable for SAR {}
#[doc = "`write(|w| ..)` method takes [sar::W](sar::W) writer structure"]
impl crate::Writable for SAR {}
#[doc = "MDMA channel x source address register"]
pub mod sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dar](dar) module"]
pub type DAR = crate::Reg<u32, _DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAR;
#[doc = "`read()` method returns [dar::R](dar::R) reader structure"]
impl crate::Readable for DAR {}
#[doc = "`write(|w| ..)` method takes [dar::W](dar::W) writer structure"]
impl crate::Writable for DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brur](brur) module"]
pub type BRUR = crate::Reg<u32, _BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRUR;
#[doc = "`read()` method returns [brur::R](brur::R) reader structure"]
impl crate::Readable for BRUR {}
#[doc = "`write(|w| ..)` method takes [brur::W](brur::W) writer structure"]
impl crate::Writable for BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lar](lar) module"]
pub type LAR = crate::Reg<u32, _LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LAR;
#[doc = "`read()` method returns [lar::R](lar::R) reader structure"]
impl crate::Readable for LAR {}
#[doc = "`write(|w| ..)` method takes [lar::W](lar::W) writer structure"]
impl crate::Writable for LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbr](tbr) module"]
pub type TBR = crate::Reg<u32, _TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBR;
#[doc = "`read()` method returns [tbr::R](tbr::R) reader structure"]
impl crate::Readable for TBR {}
#[doc = "`write(|w| ..)` method takes [tbr::W](tbr::W) writer structure"]
impl crate::Writable for TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mar](mar) module"]
pub type MAR = crate::Reg<u32, _MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAR;
#[doc = "`read()` method returns [mar::R](mar::R) reader structure"]
impl crate::Readable for MAR {}
#[doc = "`write(|w| ..)` method takes [mar::W](mar::W) writer structure"]
impl crate::Writable for MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdr](mdr) module"]
pub type MDR = crate::Reg<u32, _MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDR;
#[doc = "`read()` method returns [mdr::R](mdr::R) reader structure"]
impl crate::Readable for MDR {}
#[doc = "`write(|w| ..)` method takes [mdr::W](mdr::W) writer structure"]
impl crate::Writable for MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod mdr;
