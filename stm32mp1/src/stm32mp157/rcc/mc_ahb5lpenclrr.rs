///Register `MC_AHB5LPENCLRR` reader
pub type R = crate::R<MC_AHB5LPENCLRRrs>;
///Register `MC_AHB5LPENCLRR` writer
pub type W = crate::W<MC_AHB5LPENCLRRrs>;
///Field `GPIOZLPEN` reader - GPIOZLPEN
pub type GPIOZLPEN_R = crate::BitReader;
///Field `GPIOZLPEN` writer - GPIOZLPEN
pub type GPIOZLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYP1LPEN` reader - CRYP1LPEN
pub type CRYP1LPEN_R = crate::BitReader;
///Field `CRYP1LPEN` writer - CRYP1LPEN
pub type CRYP1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH1LPEN` reader - HASH1LPEN
pub type HASH1LPEN_R = crate::BitReader;
///Field `HASH1LPEN` writer - HASH1LPEN
pub type HASH1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNG1LPEN` reader - RNG1LPEN
pub type RNG1LPEN_R = crate::BitReader;
///Field `RNG1LPEN` writer - RNG1LPEN
pub type RNG1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPSRAMLPEN` reader - BKPSRAMLPEN
pub type BKPSRAMLPEN_R = crate::BitReader;
///Field `BKPSRAMLPEN` writer - BKPSRAMLPEN
pub type BKPSRAMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPIOZLPEN
    #[inline(always)]
    pub fn gpiozlpen(&self) -> GPIOZLPEN_R {
        GPIOZLPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - CRYP1LPEN
    #[inline(always)]
    pub fn cryp1lpen(&self) -> CRYP1LPEN_R {
        CRYP1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HASH1LPEN
    #[inline(always)]
    pub fn hash1lpen(&self) -> HASH1LPEN_R {
        HASH1LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RNG1LPEN
    #[inline(always)]
    pub fn rng1lpen(&self) -> RNG1LPEN_R {
        RNG1LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - BKPSRAMLPEN
    #[inline(always)]
    pub fn bkpsramlpen(&self) -> BKPSRAMLPEN_R {
        BKPSRAMLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MC_AHB5LPENCLRR")
            .field("gpiozlpen", &self.gpiozlpen())
            .field("cryp1lpen", &self.cryp1lpen())
            .field("hash1lpen", &self.hash1lpen())
            .field("rng1lpen", &self.rng1lpen())
            .field("bkpsramlpen", &self.bkpsramlpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOZLPEN
    #[inline(always)]
    pub fn gpiozlpen(&mut self) -> GPIOZLPEN_W<'_, MC_AHB5LPENCLRRrs> {
        GPIOZLPEN_W::new(self, 0)
    }
    ///Bit 4 - CRYP1LPEN
    #[inline(always)]
    pub fn cryp1lpen(&mut self) -> CRYP1LPEN_W<'_, MC_AHB5LPENCLRRrs> {
        CRYP1LPEN_W::new(self, 4)
    }
    ///Bit 5 - HASH1LPEN
    #[inline(always)]
    pub fn hash1lpen(&mut self) -> HASH1LPEN_W<'_, MC_AHB5LPENCLRRrs> {
        HASH1LPEN_W::new(self, 5)
    }
    ///Bit 6 - RNG1LPEN
    #[inline(always)]
    pub fn rng1lpen(&mut self) -> RNG1LPEN_W<'_, MC_AHB5LPENCLRRrs> {
        RNG1LPEN_W::new(self, 6)
    }
    ///Bit 8 - BKPSRAMLPEN
    #[inline(always)]
    pub fn bkpsramlpen(&mut self) -> BKPSRAMLPEN_W<'_, MC_AHB5LPENCLRRrs> {
        BKPSRAMLPEN_W::new(self, 8)
    }
}
/**This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mc_ahb5lpenclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb5lpenclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MC_AHB5LPENCLRR)*/
pub struct MC_AHB5LPENCLRRrs;
impl crate::RegisterSpec for MC_AHB5LPENCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`mc_ahb5lpenclrr::R`](R) reader structure
impl crate::Readable for MC_AHB5LPENCLRRrs {}
///`write(|w| ..)` method takes [`mc_ahb5lpenclrr::W`](W) writer structure
impl crate::Writable for MC_AHB5LPENCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MC_AHB5LPENCLRR to value 0x0171
impl crate::Resettable for MC_AHB5LPENCLRRrs {
    const RESET_VALUE: u32 = 0x0171;
}
