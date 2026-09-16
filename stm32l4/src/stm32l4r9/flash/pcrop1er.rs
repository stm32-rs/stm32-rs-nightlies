///Register `PCROP1ER` reader
pub type R = crate::R<PCROP1ERrs>;
///Register `PCROP1ER` writer
pub type W = crate::W<PCROP1ERrs>;
///Field `PCROP1_END` reader - Bank 1 PCROP area end offset
pub type PCROP1_END_R = crate::FieldReader<u32>;
///Field `PCROP1_END` writer - Bank 1 PCROP area end offset
pub type PCROP1_END_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32, crate::Safe>;
/**PCROP area preserved when RDP level decreased

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCROP_RDP {
    ///0: PCROP area is not erased when the RDP level is decreased from Level 1 to Level 0
    Disabled = 0,
    ///1: PCROP area is erased when the RDP level is decreased from Level 1 to Level 0
    Enabled = 1,
}
impl From<PCROP_RDP> for bool {
    #[inline(always)]
    fn from(variant: PCROP_RDP) -> Self {
        variant as u8 != 0
    }
}
///Field `PCROP_RDP` reader - PCROP area preserved when RDP level decreased
pub type PCROP_RDP_R = crate::BitReader<PCROP_RDP>;
impl PCROP_RDP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCROP_RDP {
        match self.bits {
            false => PCROP_RDP::Disabled,
            true => PCROP_RDP::Enabled,
        }
    }
    ///PCROP area is not erased when the RDP level is decreased from Level 1 to Level 0
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PCROP_RDP::Disabled
    }
    ///PCROP area is erased when the RDP level is decreased from Level 1 to Level 0
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PCROP_RDP::Enabled
    }
}
///Field `PCROP_RDP` writer - PCROP area preserved when RDP level decreased
pub type PCROP_RDP_W<'a, REG> = crate::BitWriter<'a, REG, PCROP_RDP>;
impl<'a, REG> PCROP_RDP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PCROP area is not erased when the RDP level is decreased from Level 1 to Level 0
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PCROP_RDP::Disabled)
    }
    ///PCROP area is erased when the RDP level is decreased from Level 1 to Level 0
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PCROP_RDP::Enabled)
    }
}
impl R {
    ///Bits 0:16 - Bank 1 PCROP area end offset
    #[inline(always)]
    pub fn pcrop1_end(&self) -> PCROP1_END_R {
        PCROP1_END_R::new(self.bits & 0x0001_ffff)
    }
    ///Bit 31 - PCROP area preserved when RDP level decreased
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP1ER")
            .field("pcrop1_end", &self.pcrop1_end())
            .field("pcrop_rdp", &self.pcrop_rdp())
            .finish()
    }
}
impl W {
    ///Bits 0:16 - Bank 1 PCROP area end offset
    #[inline(always)]
    pub fn pcrop1_end(&mut self) -> PCROP1_END_W<'_, PCROP1ERrs> {
        PCROP1_END_W::new(self, 0)
    }
    ///Bit 31 - PCROP area preserved when RDP level decreased
    #[inline(always)]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W<'_, PCROP1ERrs> {
        PCROP_RDP_W::new(self, 31)
    }
}
/**Flash Bank 1 PCROP End address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1er::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1er::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#FLASH:PCROP1ER)*/
pub struct PCROP1ERrs;
impl crate::RegisterSpec for PCROP1ERrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop1er::R`](R) reader structure
impl crate::Readable for PCROP1ERrs {}
///`write(|w| ..)` method takes [`pcrop1er::W`](W) writer structure
impl crate::Writable for PCROP1ERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCROP1ER to value 0x0fff_0000
impl crate::Resettable for PCROP1ERrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
