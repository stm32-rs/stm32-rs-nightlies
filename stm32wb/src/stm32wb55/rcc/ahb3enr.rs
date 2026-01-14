///Register `AHB3ENR` reader
pub type R = crate::R<AHB3ENRrs>;
///Register `AHB3ENR` writer
pub type W = crate::W<AHB3ENRrs>;
/**QSPIEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QSPIEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<QSPIEN> for bool {
    #[inline(always)]
    fn from(variant: QSPIEN) -> Self {
        variant as u8 != 0
    }
}
///Field `QSPIEN` reader - QSPIEN
pub type QSPIEN_R = crate::BitReader<QSPIEN>;
impl QSPIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> QSPIEN {
        match self.bits {
            false => QSPIEN::Disabled,
            true => QSPIEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == QSPIEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == QSPIEN::Enabled
    }
}
///Field `QSPIEN` writer - QSPIEN
pub type QSPIEN_W<'a, REG> = crate::BitWriter<'a, REG, QSPIEN>;
impl<'a, REG> QSPIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(QSPIEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(QSPIEN::Enabled)
    }
}
///Field `PKAEN` reader - PKAEN
pub use QSPIEN_R as PKAEN_R;
///Field `AES2EN` reader - AES2EN
pub use QSPIEN_R as AES2EN_R;
///Field `RNGEN` reader - RNGEN
pub use QSPIEN_R as RNGEN_R;
///Field `HSEMEN` reader - HSEMEN
pub use QSPIEN_R as HSEMEN_R;
///Field `IPCCEN` reader - IPCCEN
pub use QSPIEN_R as IPCCEN_R;
///Field `FLASHEN` reader - FLASHEN
pub use QSPIEN_R as FLASHEN_R;
///Field `PKAEN` writer - PKAEN
pub use QSPIEN_W as PKAEN_W;
///Field `AES2EN` writer - AES2EN
pub use QSPIEN_W as AES2EN_W;
///Field `RNGEN` writer - RNGEN
pub use QSPIEN_W as RNGEN_W;
///Field `HSEMEN` writer - HSEMEN
pub use QSPIEN_W as HSEMEN_W;
///Field `IPCCEN` writer - IPCCEN
pub use QSPIEN_W as IPCCEN_W;
///Field `FLASHEN` writer - FLASHEN
pub use QSPIEN_W as FLASHEN_W;
impl R {
    ///Bit 8 - QSPIEN
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - PKAEN
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AES2EN
    #[inline(always)]
    pub fn aes2en(&self) -> AES2EN_R {
        AES2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - RNGEN
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - HSEMEN
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - IPCCEN
    #[inline(always)]
    pub fn ipccen(&self) -> IPCCEN_R {
        IPCCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 25 - FLASHEN
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3ENR")
            .field("qspien", &self.qspien())
            .field("flashen", &self.flashen())
            .field("ipccen", &self.ipccen())
            .field("hsemen", &self.hsemen())
            .field("rngen", &self.rngen())
            .field("aes2en", &self.aes2en())
            .field("pkaen", &self.pkaen())
            .finish()
    }
}
impl W {
    ///Bit 8 - QSPIEN
    #[inline(always)]
    pub fn qspien(&mut self) -> QSPIEN_W<'_, AHB3ENRrs> {
        QSPIEN_W::new(self, 8)
    }
    ///Bit 16 - PKAEN
    #[inline(always)]
    pub fn pkaen(&mut self) -> PKAEN_W<'_, AHB3ENRrs> {
        PKAEN_W::new(self, 16)
    }
    ///Bit 17 - AES2EN
    #[inline(always)]
    pub fn aes2en(&mut self) -> AES2EN_W<'_, AHB3ENRrs> {
        AES2EN_W::new(self, 17)
    }
    ///Bit 18 - RNGEN
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<'_, AHB3ENRrs> {
        RNGEN_W::new(self, 18)
    }
    ///Bit 19 - HSEMEN
    #[inline(always)]
    pub fn hsemen(&mut self) -> HSEMEN_W<'_, AHB3ENRrs> {
        HSEMEN_W::new(self, 19)
    }
    ///Bit 20 - IPCCEN
    #[inline(always)]
    pub fn ipccen(&mut self) -> IPCCEN_W<'_, AHB3ENRrs> {
        IPCCEN_W::new(self, 20)
    }
    ///Bit 25 - FLASHEN
    #[inline(always)]
    pub fn flashen(&mut self) -> FLASHEN_W<'_, AHB3ENRrs> {
        FLASHEN_W::new(self, 25)
    }
}
/**AHB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:AHB3ENR)*/
pub struct AHB3ENRrs;
impl crate::RegisterSpec for AHB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3enr::R`](R) reader structure
impl crate::Readable for AHB3ENRrs {}
///`write(|w| ..)` method takes [`ahb3enr::W`](W) writer structure
impl crate::Writable for AHB3ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3ENR to value 0x0208_0000
impl crate::Resettable for AHB3ENRrs {
    const RESET_VALUE: u32 = 0x0208_0000;
}
