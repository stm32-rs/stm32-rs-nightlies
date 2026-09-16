///Register `DMAC1SFCSR` reader
pub type R = crate::R<DMAC1SFCSRrs>;
///Register `DMAC1SFCSR` writer
pub type W = crate::W<DMAC1SFCSRrs>;
///Field `ESC` reader - ESC
pub type ESC_R = crate::BitReader;
///Field `ESC` writer - ESC
pub type ESC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC` reader - ASC
pub type ASC_R = crate::BitReader;
///Field `ASC` writer - ASC
pub type ASC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSN` reader - RSN
pub type RSN_R = crate::FieldReader;
///Field `RSN` writer - RSN
pub type RSN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - ESC
    #[inline(always)]
    pub fn esc(&self) -> ESC_R {
        ESC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ASC
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 16:19 - RSN
    #[inline(always)]
    pub fn rsn(&self) -> RSN_R {
        RSN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC1SFCSR")
            .field("esc", &self.esc())
            .field("asc", &self.asc())
            .field("rsn", &self.rsn())
            .finish()
    }
}
impl W {
    ///Bit 0 - ESC
    #[inline(always)]
    pub fn esc(&mut self) -> ESC_W<'_, DMAC1SFCSRrs> {
        ESC_W::new(self, 0)
    }
    ///Bit 1 - ASC
    #[inline(always)]
    pub fn asc(&mut self) -> ASC_W<'_, DMAC1SFCSRrs> {
        ASC_W::new(self, 1)
    }
    ///Bits 16:19 - RSN
    #[inline(always)]
    pub fn rsn(&mut self) -> RSN_W<'_, DMAC1SFCSRrs> {
        RSN_W::new(self, 16)
    }
}
/**Channel i slot function control status register

You can [`read`](crate::Reg::read) this register and get [`dmac1sfcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1sfcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_DMA:DMAC1SFCSR)*/
pub struct DMAC1SFCSRrs;
impl crate::RegisterSpec for DMAC1SFCSRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac1sfcsr::R`](R) reader structure
impl crate::Readable for DMAC1SFCSRrs {}
///`write(|w| ..)` method takes [`dmac1sfcsr::W`](W) writer structure
impl crate::Writable for DMAC1SFCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC1SFCSR to value 0
impl crate::Resettable for DMAC1SFCSRrs {}
