///Register `PCROP1AER` reader
pub type R = crate::R<PCROP1AERrs>;
///Field `PCROP1A_END` reader - PCROP1A area end offset
pub type PCROP1A_END_R = crate::FieldReader;
///Field `PCROP_RDP` reader - PCROP area preserved when RDP level decreased
pub type PCROP_RDP_R = crate::BitReader;
impl R {
    ///Bits 0:7 - PCROP1A area end offset
    #[inline(always)]
    pub fn pcrop1a_end(&self) -> PCROP1A_END_R {
        PCROP1A_END_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 31 - PCROP area preserved when RDP level decreased
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
/**Flash PCROP zone A End address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1aer::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#FLASH:PCROP1AER)*/
pub struct PCROP1AERrs;
impl crate::RegisterSpec for PCROP1AERrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop1aer::R`](R) reader structure
impl crate::Readable for PCROP1AERrs {}
///`reset()` method sets PCROP1AER to value 0xf000_0000
impl crate::Resettable for PCROP1AERrs {
    const RESET_VALUE: u32 = 0xf000_0000;
}
