///Register `AHB2ENR` reader
pub type R = crate::R<AHB2ENRrs>;
///Register `AHB2ENR` writer
pub type W = crate::W<AHB2ENRrs>;
/**Camera interface enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMIEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<DCMIEN> for bool {
    #[inline(always)]
    fn from(variant: DCMIEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DCMIEN` reader - Camera interface enable
pub type DCMIEN_R = crate::BitReader<DCMIEN>;
impl DCMIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCMIEN {
        match self.bits {
            false => DCMIEN::Disabled,
            true => DCMIEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMIEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMIEN::Enabled
    }
}
///Field `DCMIEN` writer - Camera interface enable
pub type DCMIEN_W<'a, REG> = crate::BitWriter<'a, REG, DCMIEN>;
impl<'a, REG> DCMIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIEN::Enabled)
    }
}
///Field `CRYPEN` reader - Cryptographic modules clock enable
pub use DCMIEN_R as CRYPEN_R;
///Field `HASHEN` reader - Hash modules clock enable
pub use DCMIEN_R as HASHEN_R;
///Field `RNGEN` reader - Random number generator clock enable
pub use DCMIEN_R as RNGEN_R;
///Field `OTGFSEN` reader - USB OTG FS clock enable
pub use DCMIEN_R as OTGFSEN_R;
///Field `CRYPEN` writer - Cryptographic modules clock enable
pub use DCMIEN_W as CRYPEN_W;
///Field `HASHEN` writer - Hash modules clock enable
pub use DCMIEN_W as HASHEN_W;
///Field `RNGEN` writer - Random number generator clock enable
pub use DCMIEN_W as RNGEN_W;
///Field `OTGFSEN` writer - USB OTG FS clock enable
pub use DCMIEN_W as OTGFSEN_W;
impl R {
    ///Bit 0 - Camera interface enable
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Cryptographic modules clock enable
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Hash modules clock enable
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Random number generator clock enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - USB OTG FS clock enable
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2ENR")
            .field("dcmien", &self.dcmien())
            .field("otgfsen", &self.otgfsen())
            .field("rngen", &self.rngen())
            .field("hashen", &self.hashen())
            .field("crypen", &self.crypen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Camera interface enable
    #[inline(always)]
    pub fn dcmien(&mut self) -> DCMIEN_W<'_, AHB2ENRrs> {
        DCMIEN_W::new(self, 0)
    }
    ///Bit 4 - Cryptographic modules clock enable
    #[inline(always)]
    pub fn crypen(&mut self) -> CRYPEN_W<'_, AHB2ENRrs> {
        CRYPEN_W::new(self, 4)
    }
    ///Bit 5 - Hash modules clock enable
    #[inline(always)]
    pub fn hashen(&mut self) -> HASHEN_W<'_, AHB2ENRrs> {
        HASHEN_W::new(self, 5)
    }
    ///Bit 6 - Random number generator clock enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<'_, AHB2ENRrs> {
        RNGEN_W::new(self, 6)
    }
    ///Bit 7 - USB OTG FS clock enable
    #[inline(always)]
    pub fn otgfsen(&mut self) -> OTGFSEN_W<'_, AHB2ENRrs> {
        OTGFSEN_W::new(self, 7)
    }
}
/**AHB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#RCC:AHB2ENR)*/
pub struct AHB2ENRrs;
impl crate::RegisterSpec for AHB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2enr::R`](R) reader structure
impl crate::Readable for AHB2ENRrs {}
///`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure
impl crate::Writable for AHB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2ENR to value 0
impl crate::Resettable for AHB2ENRrs {}
