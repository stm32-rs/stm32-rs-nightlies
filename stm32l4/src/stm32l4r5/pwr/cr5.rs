///Register `CR5` reader
pub type R = crate::R<CR5rs>;
///Register `CR5` writer
pub type W = crate::W<CR5rs>;
/**Main regulator Range 1 mode This bit is only valid for the main regulator in Range 1 and has no effect on Range 2. It is recommended to reset this bit when the system frequency is greater than 80 MHz. Refer to Table 28: Range 1 boost mode configuration.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum R1MODE {
    ///0: Main regulator in Range 1 boost mode
    BoostMode = 0,
    ///1: Main regulator in Range 1 normal mode
    NormalMode = 1,
}
impl From<R1MODE> for bool {
    #[inline(always)]
    fn from(variant: R1MODE) -> Self {
        variant as u8 != 0
    }
}
///Field `R1MODE` reader - Main regulator Range 1 mode This bit is only valid for the main regulator in Range 1 and has no effect on Range 2. It is recommended to reset this bit when the system frequency is greater than 80 MHz. Refer to Table 28: Range 1 boost mode configuration.
pub type R1MODE_R = crate::BitReader<R1MODE>;
impl R1MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> R1MODE {
        match self.bits {
            false => R1MODE::BoostMode,
            true => R1MODE::NormalMode,
        }
    }
    ///Main regulator in Range 1 boost mode
    #[inline(always)]
    pub fn is_boost_mode(&self) -> bool {
        *self == R1MODE::BoostMode
    }
    ///Main regulator in Range 1 normal mode
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == R1MODE::NormalMode
    }
}
///Field `R1MODE` writer - Main regulator Range 1 mode This bit is only valid for the main regulator in Range 1 and has no effect on Range 2. It is recommended to reset this bit when the system frequency is greater than 80 MHz. Refer to Table 28: Range 1 boost mode configuration.
pub type R1MODE_W<'a, REG> = crate::BitWriter<'a, REG, R1MODE>;
impl<'a, REG> R1MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Main regulator in Range 1 boost mode
    #[inline(always)]
    pub fn boost_mode(self) -> &'a mut crate::W<REG> {
        self.variant(R1MODE::BoostMode)
    }
    ///Main regulator in Range 1 normal mode
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(R1MODE::NormalMode)
    }
}
impl R {
    ///Bit 8 - Main regulator Range 1 mode This bit is only valid for the main regulator in Range 1 and has no effect on Range 2. It is recommended to reset this bit when the system frequency is greater than 80 MHz. Refer to Table 28: Range 1 boost mode configuration.
    #[inline(always)]
    pub fn r1mode(&self) -> R1MODE_R {
        R1MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR5")
            .field("r1mode", &self.r1mode())
            .finish()
    }
}
impl W {
    ///Bit 8 - Main regulator Range 1 mode This bit is only valid for the main regulator in Range 1 and has no effect on Range 2. It is recommended to reset this bit when the system frequency is greater than 80 MHz. Refer to Table 28: Range 1 boost mode configuration.
    #[inline(always)]
    pub fn r1mode(&mut self) -> R1MODE_W<'_, CR5rs> {
        R1MODE_W::new(self, 8)
    }
}
/**PWR control register

You can [`read`](crate::Reg::read) this register and get [`cr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:CR5)*/
pub struct CR5rs;
impl crate::RegisterSpec for CR5rs {
    type Ux = u32;
}
///`read()` method returns [`cr5::R`](R) reader structure
impl crate::Readable for CR5rs {}
///`write(|w| ..)` method takes [`cr5::W`](W) writer structure
impl crate::Writable for CR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR5 to value 0x0100
impl crate::Resettable for CR5rs {
    const RESET_VALUE: u32 = 0x0100;
}
