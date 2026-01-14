///Register `POTTAMPRSTCR` reader
pub type R = crate::R<POTTAMPRSTCRrs>;
///Register `POTTAMPRSTCR` writer
pub type W = crate::W<POTTAMPRSTCRrs>;
///Field `POTTAMPERSETMASK` reader - This bit can be set by software to mask PKA, SAES, CRYP1/2, and HASH reset, in case of potential tamper.
pub type POTTAMPERSETMASK_R = crate::BitReader;
///Field `POTTAMPERSETMASK` writer - This bit can be set by software to mask PKA, SAES, CRYP1/2, and HASH reset, in case of potential tamper.
pub type POTTAMPERSETMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - This bit can be set by software to mask PKA, SAES, CRYP1/2, and HASH reset, in case of potential tamper.
    #[inline(always)]
    pub fn pottampersetmask(&self) -> POTTAMPERSETMASK_R {
        POTTAMPERSETMASK_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POTTAMPRSTCR")
            .field("pottampersetmask", &self.pottampersetmask())
            .finish()
    }
}
impl W {
    ///Bit 0 - This bit can be set by software to mask PKA, SAES, CRYP1/2, and HASH reset, in case of potential tamper.
    #[inline(always)]
    pub fn pottampersetmask(&mut self) -> POTTAMPERSETMASK_W<'_, POTTAMPRSTCRrs> {
        POTTAMPERSETMASK_W::new(self, 0)
    }
}
/**SYSCFG potential tamper reset register

You can [`read`](crate::Reg::read) this register and get [`pottamprstcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pottamprstcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:POTTAMPRSTCR)*/
pub struct POTTAMPRSTCRrs;
impl crate::RegisterSpec for POTTAMPRSTCRrs {
    type Ux = u32;
}
///`read()` method returns [`pottamprstcr::R`](R) reader structure
impl crate::Readable for POTTAMPRSTCRrs {}
///`write(|w| ..)` method takes [`pottamprstcr::W`](W) writer structure
impl crate::Writable for POTTAMPRSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets POTTAMPRSTCR to value 0
impl crate::Resettable for POTTAMPRSTCRrs {}
