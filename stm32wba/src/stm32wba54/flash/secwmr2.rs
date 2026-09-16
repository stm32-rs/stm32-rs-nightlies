///Register `SECWMR2` reader
pub type R = crate::R<SECWMR2rs>;
///Register `SECWMR2` writer
pub type W = crate::W<SECWMR2rs>;
///Field `HDP_PEND` reader - End page of secure hide protection area This field contains the last page of the secure HDP area.
pub type HDP_PEND_R = crate::FieldReader;
///Field `HDP_PEND` writer - End page of secure hide protection area This field contains the last page of the secure HDP area.
pub type HDP_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `HDPEN` reader - Secure Hide protection area enable
pub type HDPEN_R = crate::BitReader;
///Field `HDPEN` writer - Secure Hide protection area enable
pub type HDPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 16:22 - End page of secure hide protection area This field contains the last page of the secure HDP area.
    #[inline(always)]
    pub fn hdp_pend(&self) -> HDP_PEND_R {
        HDP_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 31 - Secure Hide protection area enable
    #[inline(always)]
    pub fn hdpen(&self) -> HDPEN_R {
        HDPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECWMR2")
            .field("hdp_pend", &self.hdp_pend())
            .field("hdpen", &self.hdpen())
            .finish()
    }
}
impl W {
    ///Bits 16:22 - End page of secure hide protection area This field contains the last page of the secure HDP area.
    #[inline(always)]
    pub fn hdp_pend(&mut self) -> HDP_PEND_W<'_, SECWMR2rs> {
        HDP_PEND_W::new(self, 16)
    }
    ///Bit 31 - Secure Hide protection area enable
    #[inline(always)]
    pub fn hdpen(&mut self) -> HDPEN_W<'_, SECWMR2rs> {
        HDPEN_W::new(self, 31)
    }
}
/**FLASH secure watermark register 2

You can [`read`](crate::Reg::read) this register and get [`secwmr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwmr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:SECWMR2)*/
pub struct SECWMR2rs;
impl crate::RegisterSpec for SECWMR2rs {
    type Ux = u32;
}
///`read()` method returns [`secwmr2::R`](R) reader structure
impl crate::Readable for SECWMR2rs {}
///`write(|w| ..)` method takes [`secwmr2::W`](W) writer structure
impl crate::Writable for SECWMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECWMR2 to value 0x0f00_0f00
impl crate::Resettable for SECWMR2rs {
    const RESET_VALUE: u32 = 0x0f00_0f00;
}
