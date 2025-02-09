///Register `PCROP1AER` reader
pub type R = crate::R<PCROP1AERrs>;
///Register `PCROP1AER` writer
pub type W = crate::W<PCROP1AERrs>;
///Field `PCROP1A_END` reader - PCROP1A area end offset Contains the offset of the last subpage of the PCROP1A area. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type PCROP1A_END_R = crate::FieldReader;
///Field `PCROP1A_END` writer - PCROP1A area end offset Contains the offset of the last subpage of the PCROP1A area. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type PCROP1A_END_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PCROP_RDP` reader - PCROP area erase upon RDP level regression This bit determines whether the PCROP area (and the totality of the PCROP area boundary pages) is erased by the mass erase triggered by the RDP level regression from Level 1 to Level 0: The software can only set this bit. It is automatically reset upon mass erase following the RDP regression from Level 1 to Level 0.
pub type PCROP_RDP_R = crate::BitReader;
///Field `PCROP_RDP` writer - PCROP area erase upon RDP level regression This bit determines whether the PCROP area (and the totality of the PCROP area boundary pages) is erased by the mass erase triggered by the RDP level regression from Level 1 to Level 0: The software can only set this bit. It is automatically reset upon mass erase following the RDP regression from Level 1 to Level 0.
pub type PCROP_RDP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - PCROP1A area end offset Contains the offset of the last subpage of the PCROP1A area. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn pcrop1a_end(&self) -> PCROP1A_END_R {
        PCROP1A_END_R::new((self.bits & 0xff) as u8)
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
    ///Bits 0:7 - PCROP1A area end offset Contains the offset of the last subpage of the PCROP1A area. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn pcrop1a_end(&mut self) -> PCROP1A_END_W<PCROP1AERrs> {
        PCROP1A_END_W::new(self, 0)
    }
    ///Bit 31 - PCROP area erase upon RDP level regression This bit determines whether the PCROP area (and the totality of the PCROP area boundary pages) is erased by the mass erase triggered by the RDP level regression from Level 1 to Level 0: The software can only set this bit. It is automatically reset upon mass erase following the RDP regression from Level 1 to Level 0.
    #[inline(always)]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W<PCROP1AERrs> {
        PCROP_RDP_W::new(self, 31)
    }
}
/**FLASH PCROP area A end address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1aer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1aer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#FLASH:PCROP1AER)*/
pub struct PCROP1AERrs;
impl crate::RegisterSpec for PCROP1AERrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop1aer::R`](R) reader structure
impl crate::Readable for PCROP1AERrs {}
///`write(|w| ..)` method takes [`pcrop1aer::W`](W) writer structure
impl crate::Writable for PCROP1AERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PCROP1AER to value 0
impl crate::Resettable for PCROP1AERrs {
    const RESET_VALUE: u32 = 0;
}
