///Register `DFSDM_FLT1AWHTR` reader
pub type R = crate::R<DFSDM_FLT1AWHTRrs>;
///Register `DFSDM_FLT1AWHTR` writer
pub type W = crate::W<DFSDM_FLT1AWHTRrs>;
///Field `BKAWH` reader - BKAWH
pub type BKAWH_R = crate::FieldReader;
///Field `BKAWH` writer - BKAWH
pub type BKAWH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AWHT` reader - AWHT
pub type AWHT_R = crate::FieldReader<u32>;
///Field `AWHT` writer - AWHT
pub type AWHT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:3 - BKAWH
    #[inline(always)]
    pub fn bkawh(&self) -> BKAWH_R {
        BKAWH_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:31 - AWHT
    #[inline(always)]
    pub fn awht(&self) -> AWHT_R {
        AWHT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFSDM_FLT1AWHTR")
            .field("bkawh", &self.bkawh())
            .field("awht", &self.awht())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - BKAWH
    #[inline(always)]
    #[must_use]
    pub fn bkawh(&mut self) -> BKAWH_W<DFSDM_FLT1AWHTRrs> {
        BKAWH_W::new(self, 0)
    }
    ///Bits 8:31 - AWHT
    #[inline(always)]
    #[must_use]
    pub fn awht(&mut self) -> AWHT_W<DFSDM_FLT1AWHTRrs> {
        AWHT_W::new(self, 8)
    }
}
/**DFSDM filter 1 analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1awhtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1awhtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT1AWHTR)*/
pub struct DFSDM_FLT1AWHTRrs;
impl crate::RegisterSpec for DFSDM_FLT1AWHTRrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm_flt1awhtr::R`](R) reader structure
impl crate::Readable for DFSDM_FLT1AWHTRrs {}
///`write(|w| ..)` method takes [`dfsdm_flt1awhtr::W`](W) writer structure
impl crate::Writable for DFSDM_FLT1AWHTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DFSDM_FLT1AWHTR to value 0
impl crate::Resettable for DFSDM_FLT1AWHTRrs {
    const RESET_VALUE: u32 = 0;
}
