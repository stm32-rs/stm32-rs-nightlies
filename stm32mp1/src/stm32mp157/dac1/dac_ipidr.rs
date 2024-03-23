#[doc = "Register `DAC_IPIDR` reader"]
pub type R = crate::R<DAC_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "No\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC_IPIDRrs;
impl crate::RegisterSpec for DAC_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_ipidr::R`](R) reader structure"]
impl crate::Readable for DAC_IPIDRrs {}
#[doc = "`reset()` method sets DAC_IPIDR to value 0x0011_0011"]
impl crate::Resettable for DAC_IPIDRrs {
    const RESET_VALUE: u32 = 0x0011_0011;
}
