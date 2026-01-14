///Register `PCROP1AER` reader
pub type R = crate::R<PCROP1AERrs>;
///Register `PCROP1AER` writer
pub type W = crate::W<PCROP1AERrs>;
///Field `PCROP1A_END` reader - PCROP1A area end offset Contains the offset of the last subpage of the PCROP1A area. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type PCROP1A_END_R = crate::FieldReader<u16>;
///Field `PCROP1A_END` writer - PCROP1A area end offset Contains the offset of the last subpage of the PCROP1A area. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type PCROP1A_END_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
/**PCROP area erase upon RDP level regression This bit determines whether the PCROP area (and the totality of the PCROP area boundary pages) is erased by the mass erase triggered by the RDP level regression from Level 1 to Level 0: The software can only set this bit. It is automatically reset upon mass erase following the RDP regression from Level 1 to Level 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCROP_RDP {
    ///0: Not erased
    B0x0 = 0,
    ///1: Erased
    B0x1 = 1,
}
impl From<PCROP_RDP> for bool {
    #[inline(always)]
    fn from(variant: PCROP_RDP) -> Self {
        variant as u8 != 0
    }
}
///Field `PCROP_RDP` reader - PCROP area erase upon RDP level regression This bit determines whether the PCROP area (and the totality of the PCROP area boundary pages) is erased by the mass erase triggered by the RDP level regression from Level 1 to Level 0: The software can only set this bit. It is automatically reset upon mass erase following the RDP regression from Level 1 to Level 0.
pub type PCROP_RDP_R = crate::BitReader<PCROP_RDP>;
impl PCROP_RDP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCROP_RDP {
        match self.bits {
            false => PCROP_RDP::B0x0,
            true => PCROP_RDP::B0x1,
        }
    }
    ///Not erased
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PCROP_RDP::B0x0
    }
    ///Erased
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PCROP_RDP::B0x1
    }
}
///Field `PCROP_RDP` writer - PCROP area erase upon RDP level regression This bit determines whether the PCROP area (and the totality of the PCROP area boundary pages) is erased by the mass erase triggered by the RDP level regression from Level 1 to Level 0: The software can only set this bit. It is automatically reset upon mass erase following the RDP regression from Level 1 to Level 0.
pub type PCROP_RDP_W<'a, REG> = crate::BitWriter<'a, REG, PCROP_RDP>;
impl<'a, REG> PCROP_RDP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not erased
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PCROP_RDP::B0x0)
    }
    ///Erased
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PCROP_RDP::B0x1)
    }
}
impl R {
    ///Bits 0:8 - PCROP1A area end offset Contains the offset of the last subpage of the PCROP1A area. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn pcrop1a_end(&self) -> PCROP1A_END_R {
        PCROP1A_END_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 31 - PCROP area erase upon RDP level regression This bit determines whether the PCROP area (and the totality of the PCROP area boundary pages) is erased by the mass erase triggered by the RDP level regression from Level 1 to Level 0: The software can only set this bit. It is automatically reset upon mass erase following the RDP regression from Level 1 to Level 0.
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP1AER")
            .field("pcrop1a_end", &self.pcrop1a_end())
            .field("pcrop_rdp", &self.pcrop_rdp())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - PCROP1A area end offset Contains the offset of the last subpage of the PCROP1A area. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn pcrop1a_end(&mut self) -> PCROP1A_END_W<'_, PCROP1AERrs> {
        PCROP1A_END_W::new(self, 0)
    }
    ///Bit 31 - PCROP area erase upon RDP level regression This bit determines whether the PCROP area (and the totality of the PCROP area boundary pages) is erased by the mass erase triggered by the RDP level regression from Level 1 to Level 0: The software can only set this bit. It is automatically reset upon mass erase following the RDP regression from Level 1 to Level 0.
    #[inline(always)]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W<'_, PCROP1AERrs> {
        PCROP_RDP_W::new(self, 31)
    }
}
/**FLASH PCROP area A end address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1aer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1aer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#FLASH:PCROP1AER)*/
pub struct PCROP1AERrs;
impl crate::RegisterSpec for PCROP1AERrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop1aer::R`](R) reader structure
impl crate::Readable for PCROP1AERrs {}
///`write(|w| ..)` method takes [`pcrop1aer::W`](W) writer structure
impl crate::Writable for PCROP1AERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCROP1AER to value 0
impl crate::Resettable for PCROP1AERrs {}
