///Register `GVCIDR` reader
pub type R = crate::R<GVCIDRrs>;
///Field `VCID` reader - Virtual channel ID
pub type VCID_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - Virtual channel ID
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GVCIDR")
            .field("vcid", &self.vcid())
            .finish()
    }
}
/**DSI Host generic VCID register

You can [`read`](crate::Reg::read) this register and get [`gvcidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#DSIHOST:GVCIDR)*/
pub struct GVCIDRrs;
impl crate::RegisterSpec for GVCIDRrs {
    type Ux = u32;
}
///`read()` method returns [`gvcidr::R`](R) reader structure
impl crate::Readable for GVCIDRrs {}
///`reset()` method sets GVCIDR to value 0
impl crate::Resettable for GVCIDRrs {}
