///Register `HWCFGR4` reader
pub type R = crate::R<HWCFGR4rs>;
///Register `HWCFGR4` writer
pub type W = crate::W<HWCFGR4rs>;
///Field `CHMAP15` reader - Input channel mapping
pub type CHMAP15_R = crate::FieldReader;
///Field `CHMAP15` writer - Input channel mapping
pub type CHMAP15_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CHMAP14` reader - Input channel mapping
pub type CHMAP14_R = crate::FieldReader;
///Field `CHMAP14` writer - Input channel mapping
pub type CHMAP14_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CHMAP13` reader - Input channel mapping
pub type CHMAP13_R = crate::FieldReader;
///Field `CHMAP13` writer - Input channel mapping
pub type CHMAP13_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CHMAP12` reader - Input channel mapping
pub type CHMAP12_R = crate::FieldReader;
///Field `CHMAP12` writer - Input channel mapping
pub type CHMAP12_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Input channel mapping
    #[inline(always)]
    pub fn chmap15(&self) -> CHMAP15_R {
        CHMAP15_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - Input channel mapping
    #[inline(always)]
    pub fn chmap14(&self) -> CHMAP14_R {
        CHMAP14_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - Input channel mapping
    #[inline(always)]
    pub fn chmap13(&self) -> CHMAP13_R {
        CHMAP13_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:28 - Input channel mapping
    #[inline(always)]
    pub fn chmap12(&self) -> CHMAP12_R {
        CHMAP12_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR4")
            .field("chmap15", &self.chmap15())
            .field("chmap14", &self.chmap14())
            .field("chmap13", &self.chmap13())
            .field("chmap12", &self.chmap12())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Input channel mapping
    #[inline(always)]
    #[must_use]
    pub fn chmap15(&mut self) -> CHMAP15_W<HWCFGR4rs> {
        CHMAP15_W::new(self, 0)
    }
    ///Bits 8:12 - Input channel mapping
    #[inline(always)]
    #[must_use]
    pub fn chmap14(&mut self) -> CHMAP14_W<HWCFGR4rs> {
        CHMAP14_W::new(self, 8)
    }
    ///Bits 16:20 - Input channel mapping
    #[inline(always)]
    #[must_use]
    pub fn chmap13(&mut self) -> CHMAP13_W<HWCFGR4rs> {
        CHMAP13_W::new(self, 16)
    }
    ///Bits 24:28 - Input channel mapping
    #[inline(always)]
    #[must_use]
    pub fn chmap12(&mut self) -> CHMAP12_W<HWCFGR4rs> {
        CHMAP12_W::new(self, 24)
    }
}
/**Hardware Configuration Register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwcfgr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#ADC:HWCFGR4)*/
pub struct HWCFGR4rs;
impl crate::RegisterSpec for HWCFGR4rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr4::R`](R) reader structure
impl crate::Readable for HWCFGR4rs {}
///`write(|w| ..)` method takes [`hwcfgr4::W`](W) writer structure
impl crate::Writable for HWCFGR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HWCFGR4 to value 0x070b_0a09
impl crate::Resettable for HWCFGR4rs {
    const RESET_VALUE: u32 = 0x070b_0a09;
}
