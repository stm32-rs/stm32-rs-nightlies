///Register `HSECR` reader
pub type R = crate::R<HSECRrs>;
///Register `HSECR` writer
pub type W = crate::W<HSECRrs>;
///Field `UNLOCKED` reader - Register lock system
pub type UNLOCKED_R = crate::BitReader;
///Field `UNLOCKED` writer - Register lock system
pub type UNLOCKED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSES` reader - HSE Sense amplifier threshold
pub type HSES_R = crate::BitReader;
///Field `HSES` writer - HSE Sense amplifier threshold
pub type HSES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEGMC` reader - HSE current control
pub type HSEGMC_R = crate::FieldReader;
///Field `HSEGMC` writer - HSE current control
pub type HSEGMC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HSETUNE` reader - HSE capacitor tuning
pub type HSETUNE_R = crate::FieldReader;
impl R {
    ///Bit 0 - Register lock system
    #[inline(always)]
    pub fn unlocked(&self) -> UNLOCKED_R {
        UNLOCKED_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - HSE Sense amplifier threshold
    #[inline(always)]
    pub fn hses(&self) -> HSES_R {
        HSES_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - HSE current control
    #[inline(always)]
    pub fn hsegmc(&self) -> HSEGMC_R {
        HSEGMC_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:13 - HSE capacitor tuning
    #[inline(always)]
    pub fn hsetune(&self) -> HSETUNE_R {
        HSETUNE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSECR")
            .field("hsetune", &self.hsetune())
            .field("hsegmc", &self.hsegmc())
            .field("hses", &self.hses())
            .field("unlocked", &self.unlocked())
            .finish()
    }
}
impl W {
    ///Bit 0 - Register lock system
    #[inline(always)]
    #[must_use]
    pub fn unlocked(&mut self) -> UNLOCKED_W<HSECRrs> {
        UNLOCKED_W::new(self, 0)
    }
    ///Bit 3 - HSE Sense amplifier threshold
    #[inline(always)]
    #[must_use]
    pub fn hses(&mut self) -> HSES_W<HSECRrs> {
        HSES_W::new(self, 3)
    }
    ///Bits 4:6 - HSE current control
    #[inline(always)]
    #[must_use]
    pub fn hsegmc(&mut self) -> HSEGMC_W<HSECRrs> {
        HSEGMC_W::new(self, 4)
    }
}
/**Clock HSE register

You can [`read`](crate::Reg::read) this register and get [`hsecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:HSECR)*/
pub struct HSECRrs;
impl crate::RegisterSpec for HSECRrs {
    type Ux = u32;
}
///`read()` method returns [`hsecr::R`](R) reader structure
impl crate::Readable for HSECRrs {}
///`write(|w| ..)` method takes [`hsecr::W`](W) writer structure
impl crate::Writable for HSECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HSECR to value 0x30
impl crate::Resettable for HSECRrs {
    const RESET_VALUE: u32 = 0x30;
}
