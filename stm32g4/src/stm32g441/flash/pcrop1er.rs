///Register `PCROP1ER` reader
pub type R = crate::R<PCROP1ERrs>;
///Register `PCROP1ER` writer
pub type W = crate::W<PCROP1ERrs>;
///Field `PCROP1_END` reader - PCROP area end offset
pub type PCROP1_END_R = crate::FieldReader<u16>;
///Field `PCROP1_END` writer - PCROP area end offset
pub type PCROP1_END_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `PCROP_RDP` reader - PCROP area preserved when RDP level decreased
pub type PCROP_RDP_R = crate::BitReader;
///Field `PCROP_RDP` writer - PCROP area preserved when RDP level decreased
pub type PCROP_RDP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:14 - PCROP area end offset
    #[inline(always)]
    pub fn pcrop1_end(&self) -> PCROP1_END_R {
        PCROP1_END_R::new((self.bits & 0x7fff) as u16)
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
    ///Bits 0:14 - PCROP area end offset
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
/**Flash PCROP1 End address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1er::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1er::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G441.html#FLASH:PCROP1ER)*/
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
///`reset()` method sets PCROP1ER to value 0
impl crate::Resettable for PCROP1ERrs {}
