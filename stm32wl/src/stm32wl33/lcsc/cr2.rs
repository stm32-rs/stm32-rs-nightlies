///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `TAMP_PSC` reader - Tamper measurement interval.
pub type TAMP_PSC_R = crate::FieldReader;
///Field `TAMP_PSC` writer - Tamper measurement interval.
pub type TAMP_PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LCT_DAMP_THRES` reader - Damping threshold for LCT
pub type LCT_DAMP_THRES_R = crate::FieldReader;
///Field `LCT_DAMP_THRES` writer - Damping threshold for LCT
pub type LCT_DAMP_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Tamper measurement interval.
    #[inline(always)]
    pub fn tamp_psc(&self) -> TAMP_PSC_R {
        TAMP_PSC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Damping threshold for LCT
    #[inline(always)]
    pub fn lct_damp_thres(&self) -> LCT_DAMP_THRES_R {
        LCT_DAMP_THRES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("tamp_psc", &self.tamp_psc())
            .field("lct_damp_thres", &self.lct_damp_thres())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Tamper measurement interval.
    #[inline(always)]
    pub fn tamp_psc(&mut self) -> TAMP_PSC_W<'_, CR2rs> {
        TAMP_PSC_W::new(self, 0)
    }
    ///Bits 8:15 - Damping threshold for LCT
    #[inline(always)]
    pub fn lct_damp_thres(&mut self) -> LCT_DAMP_THRES_W<'_, CR2rs> {
        LCT_DAMP_THRES_W::new(self, 8)
    }
}
/**LCSC_CR2 register

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0x8000
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0x8000;
}
