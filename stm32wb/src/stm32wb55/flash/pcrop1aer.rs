///Register `PCROP1AER` reader
pub type R = crate::R<PCROP1AERrs>;
///Register `PCROP1AER` writer
pub type W = crate::W<PCROP1AERrs>;
///Field `PCROP1A_END` reader - Bank 1 PCROP area end offset
pub type PCROP1A_END_R = crate::FieldReader<u16>;
///Field `PCROP1A_END` writer - Bank 1 PCROP area end offset
pub type PCROP1A_END_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `PCROP_RDP` reader - PCROP area preserved when RDP level decreased
pub type PCROP_RDP_R = crate::BitReader;
///Field `PCROP_RDP` writer - PCROP area preserved when RDP level decreased
pub type PCROP_RDP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:8 - Bank 1 PCROP area end offset
    #[inline(always)]
    pub fn pcrop1a_end(&self) -> PCROP1A_END_R {
        PCROP1A_END_R::new((self.bits & 0x01ff) as u16)
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
impl W {
    ///Bits 0:8 - Bank 1 PCROP area end offset
    #[inline(always)]
    pub fn pcrop1a_end(&mut self) -> PCROP1A_END_W<'_, PCROP1AERrs> {
        PCROP1A_END_W::new(self, 0)
    }
    ///Bit 31 - PCROP area preserved when RDP level decreased
    #[inline(always)]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W<'_, PCROP1AERrs> {
        PCROP_RDP_W::new(self, 31)
    }
}
/**Flash Bank 1 PCROP End address zone A register

You can [`read`](crate::Reg::read) this register and get [`pcrop1aer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1aer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#Flash:PCROP1AER)*/
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
///`reset()` method sets PCROP1AER to value 0x7fff_fe00
impl crate::Resettable for PCROP1AERrs {
    const RESET_VALUE: u32 = 0x7fff_fe00;
}
