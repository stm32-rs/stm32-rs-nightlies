///Register `AHB5RSTSETR` reader
pub type R = crate::R<AHB5RSTSETRrs>;
///Register `AHB5RSTSETR` writer
pub type W = crate::W<AHB5RSTSETRrs>;
///Field `GPIOZRST` reader - GPIOZRST
pub type GPIOZRST_R = crate::BitReader;
///Field `GPIOZRST` writer - GPIOZRST
pub type GPIOZRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYP1RST` reader - CRYP1RST
pub type CRYP1RST_R = crate::BitReader;
///Field `CRYP1RST` writer - CRYP1RST
pub type CRYP1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH1RST` reader - HASH1RST
pub type HASH1RST_R = crate::BitReader;
///Field `HASH1RST` writer - HASH1RST
pub type HASH1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNG1RST` reader - RNG1RST
pub type RNG1RST_R = crate::BitReader;
///Field `RNG1RST` writer - RNG1RST
pub type RNG1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXIMCRST` reader - AXIMCRST
pub type AXIMCRST_R = crate::BitReader;
///Field `AXIMCRST` writer - AXIMCRST
pub type AXIMCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPIOZRST
    #[inline(always)]
    pub fn gpiozrst(&self) -> GPIOZRST_R {
        GPIOZRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - CRYP1RST
    #[inline(always)]
    pub fn cryp1rst(&self) -> CRYP1RST_R {
        CRYP1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HASH1RST
    #[inline(always)]
    pub fn hash1rst(&self) -> HASH1RST_R {
        HASH1RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RNG1RST
    #[inline(always)]
    pub fn rng1rst(&self) -> RNG1RST_R {
        RNG1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - AXIMCRST
    #[inline(always)]
    pub fn aximcrst(&self) -> AXIMCRST_R {
        AXIMCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB5RSTSETR")
            .field("gpiozrst", &self.gpiozrst())
            .field("cryp1rst", &self.cryp1rst())
            .field("hash1rst", &self.hash1rst())
            .field("rng1rst", &self.rng1rst())
            .field("aximcrst", &self.aximcrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOZRST
    #[inline(always)]
    pub fn gpiozrst(&mut self) -> GPIOZRST_W<'_, AHB5RSTSETRrs> {
        GPIOZRST_W::new(self, 0)
    }
    ///Bit 4 - CRYP1RST
    #[inline(always)]
    pub fn cryp1rst(&mut self) -> CRYP1RST_W<'_, AHB5RSTSETRrs> {
        CRYP1RST_W::new(self, 4)
    }
    ///Bit 5 - HASH1RST
    #[inline(always)]
    pub fn hash1rst(&mut self) -> HASH1RST_W<'_, AHB5RSTSETRrs> {
        HASH1RST_W::new(self, 5)
    }
    ///Bit 6 - RNG1RST
    #[inline(always)]
    pub fn rng1rst(&mut self) -> RNG1RST_W<'_, AHB5RSTSETRrs> {
        RNG1RST_W::new(self, 6)
    }
    ///Bit 16 - AXIMCRST
    #[inline(always)]
    pub fn aximcrst(&mut self) -> AXIMCRST_W<'_, AHB5RSTSETRrs> {
        AXIMCRST_W::new(self, 16)
    }
}
/**This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`ahb5rstsetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5rstsetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:AHB5RSTSETR)*/
pub struct AHB5RSTSETRrs;
impl crate::RegisterSpec for AHB5RSTSETRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb5rstsetr::R`](R) reader structure
impl crate::Readable for AHB5RSTSETRrs {}
///`write(|w| ..)` method takes [`ahb5rstsetr::W`](W) writer structure
impl crate::Writable for AHB5RSTSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB5RSTSETR to value 0
impl crate::Resettable for AHB5RSTSETRrs {}
