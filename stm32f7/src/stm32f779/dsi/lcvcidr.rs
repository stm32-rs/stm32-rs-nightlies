///Register `LCVCIDR` reader
pub type R = crate::R<LCVCIDRrs>;
///Field `VCID` reader - Virtual Channel ID
pub type VCID_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - Virtual Channel ID
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCVCIDR")
            .field("vcid", &self.vcid())
            .finish()
    }
}
/**DSI Host LTDC Current VCID Register

You can [`read`](crate::Reg::read) this register and get [`lcvcidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#DSI:LCVCIDR)*/
pub struct LCVCIDRrs;
impl crate::RegisterSpec for LCVCIDRrs {
    type Ux = u32;
}
///`read()` method returns [`lcvcidr::R`](R) reader structure
impl crate::Readable for LCVCIDRrs {}
///`reset()` method sets LCVCIDR to value 0
impl crate::Resettable for LCVCIDRrs {}
