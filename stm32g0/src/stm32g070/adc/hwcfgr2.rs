///Register `HWCFGR2` reader
pub type R = crate::R<HWCFGR2rs>;
///Register `HWCFGR2` writer
pub type W = crate::W<HWCFGR2rs>;
///Field `CHMAP7` reader - Input channel mapping
pub type CHMAP7_R = crate::FieldReader;
///Field `CHMAP7` writer - Input channel mapping
pub type CHMAP7_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CHMAP6` reader - Input channel mapping
pub type CHMAP6_R = crate::FieldReader;
///Field `CHMAP6` writer - Input channel mapping
pub type CHMAP6_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CHMAP5` reader - Input channel mapping
pub type CHMAP5_R = crate::FieldReader;
///Field `CHMAP5` writer - Input channel mapping
pub type CHMAP5_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CHMAP4` reader - Input channel mapping
pub type CHMAP4_R = crate::FieldReader;
///Field `CHMAP4` writer - Input channel mapping
pub type CHMAP4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Input channel mapping
    #[inline(always)]
    pub fn chmap7(&self) -> CHMAP7_R {
        CHMAP7_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - Input channel mapping
    #[inline(always)]
    pub fn chmap6(&self) -> CHMAP6_R {
        CHMAP6_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - Input channel mapping
    #[inline(always)]
    pub fn chmap5(&self) -> CHMAP5_R {
        CHMAP5_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:28 - Input channel mapping
    #[inline(always)]
    pub fn chmap4(&self) -> CHMAP4_R {
        CHMAP4_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR2")
            .field("chmap7", &self.chmap7())
            .field("chmap6", &self.chmap6())
            .field("chmap5", &self.chmap5())
            .field("chmap4", &self.chmap4())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Input channel mapping
    #[inline(always)]
    #[must_use]
    pub fn chmap7(&mut self) -> CHMAP7_W<HWCFGR2rs> {
        CHMAP7_W::new(self, 0)
    }
    ///Bits 8:12 - Input channel mapping
    #[inline(always)]
    #[must_use]
    pub fn chmap6(&mut self) -> CHMAP6_W<HWCFGR2rs> {
        CHMAP6_W::new(self, 8)
    }
    ///Bits 16:20 - Input channel mapping
    #[inline(always)]
    #[must_use]
    pub fn chmap5(&mut self) -> CHMAP5_W<HWCFGR2rs> {
        CHMAP5_W::new(self, 16)
    }
    ///Bits 24:28 - Input channel mapping
    #[inline(always)]
    #[must_use]
    pub fn chmap4(&mut self) -> CHMAP4_W<HWCFGR2rs> {
        CHMAP4_W::new(self, 24)
    }
}
/**Hardware Configuration Register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:HWCFGR2)*/
pub struct HWCFGR2rs;
impl crate::RegisterSpec for HWCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr2::R`](R) reader structure
impl crate::Readable for HWCFGR2rs {}
///`write(|w| ..)` method takes [`hwcfgr2::W`](W) writer structure
impl crate::Writable for HWCFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HWCFGR2 to value 0x0505_0404
impl crate::Resettable for HWCFGR2rs {
    const RESET_VALUE: u32 = 0x0505_0404;
}
