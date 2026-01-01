///Register `SECWMR1` reader
pub type R = crate::R<SECWMR1rs>;
///Register `SECWMR1` writer
pub type W = crate::W<SECWMR1rs>;
///Field `SECWM_PSTRT` reader - Start page of secure area This field contains the first page of the secure area.
pub type SECWM_PSTRT_R = crate::FieldReader;
///Field `SECWM_PSTRT` writer - Start page of secure area This field contains the first page of the secure area.
pub type SECWM_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SECWM_PEND` reader - End page of secure area This field contains the last page of the secure area.
pub type SECWM_PEND_R = crate::FieldReader;
///Field `SECWM_PEND` writer - End page of secure area This field contains the last page of the secure area.
pub type SECWM_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - Start page of secure area This field contains the first page of the secure area.
    #[inline(always)]
    pub fn secwm_pstrt(&self) -> SECWM_PSTRT_R {
        SECWM_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - End page of secure area This field contains the last page of the secure area.
    #[inline(always)]
    pub fn secwm_pend(&self) -> SECWM_PEND_R {
        SECWM_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECWMR1")
            .field("secwm_pstrt", &self.secwm_pstrt())
            .field("secwm_pend", &self.secwm_pend())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Start page of secure area This field contains the first page of the secure area.
    #[inline(always)]
    pub fn secwm_pstrt(&mut self) -> SECWM_PSTRT_W<'_, SECWMR1rs> {
        SECWM_PSTRT_W::new(self, 0)
    }
    ///Bits 16:22 - End page of secure area This field contains the last page of the secure area.
    #[inline(always)]
    pub fn secwm_pend(&mut self) -> SECWM_PEND_W<'_, SECWMR1rs> {
        SECWM_PEND_W::new(self, 16)
    }
}
/**FLASH secure watermark register 1

You can [`read`](crate::Reg::read) this register and get [`secwmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#FLASH:SECWMR1)*/
pub struct SECWMR1rs;
impl crate::RegisterSpec for SECWMR1rs {
    type Ux = u32;
}
///`read()` method returns [`secwmr1::R`](R) reader structure
impl crate::Readable for SECWMR1rs {}
///`write(|w| ..)` method takes [`secwmr1::W`](W) writer structure
impl crate::Writable for SECWMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECWMR1 to value 0xff00_ff00
impl crate::Resettable for SECWMR1rs {
    const RESET_VALUE: u32 = 0xff00_ff00;
}
