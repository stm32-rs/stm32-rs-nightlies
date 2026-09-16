///Register `WHEEL_SR` reader
pub type R = crate::R<WHEEL_SRrs>;
///Field `CLKWISE` reader - Number of Clock Wise revolutions
pub type CLKWISE_R = crate::FieldReader<u16>;
///Field `ACLKWISE` reader - Number of Anti Clock Wise revolutions
pub type ACLKWISE_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Number of Clock Wise revolutions
    #[inline(always)]
    pub fn clkwise(&self) -> CLKWISE_R {
        CLKWISE_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Number of Anti Clock Wise revolutions
    #[inline(always)]
    pub fn aclkwise(&self) -> ACLKWISE_R {
        ACLKWISE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WHEEL_SR")
            .field("clkwise", &self.clkwise())
            .field("aclkwise", &self.aclkwise())
            .finish()
    }
}
/**LCSC_WHEEL_SR register

You can [`read`](crate::Reg::read) this register and get [`wheel_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:WHEEL_SR)*/
pub struct WHEEL_SRrs;
impl crate::RegisterSpec for WHEEL_SRrs {
    type Ux = u32;
}
///`read()` method returns [`wheel_sr::R`](R) reader structure
impl crate::Readable for WHEEL_SRrs {}
///`reset()` method sets WHEEL_SR to value 0
impl crate::Resettable for WHEEL_SRrs {}
