///Register `PRAR_PRG` reader
pub type R = crate::R<PRAR_PRGrs>;
///Register `PRAR_PRG` writer
pub type W = crate::W<PRAR_PRGrs>;
///Field `PROT_AREA_START` reader - lowest PCROP protected address configuration
pub type PROT_AREA_START_R = crate::FieldReader<u16>;
///Field `PROT_AREA_START` writer - lowest PCROP protected address configuration
pub type PROT_AREA_START_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `PROT_AREA_END` reader - highest PCROP protected address configuration
pub type PROT_AREA_END_R = crate::FieldReader<u16>;
///Field `PROT_AREA_END` writer - highest PCROP protected address configuration
pub type PROT_AREA_END_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `DMEP` reader - PCROP protected erase enable option configuration bit
pub type DMEP_R = crate::BitReader;
///Field `DMEP` writer - PCROP protected erase enable option configuration bit
pub type DMEP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - lowest PCROP protected address configuration
    #[inline(always)]
    pub fn prot_area_start(&self) -> PROT_AREA_START_R {
        PROT_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - highest PCROP protected address configuration
    #[inline(always)]
    pub fn prot_area_end(&self) -> PROT_AREA_END_R {
        PROT_AREA_END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - PCROP protected erase enable option configuration bit
    #[inline(always)]
    pub fn dmep(&self) -> DMEP_R {
        DMEP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRAR_PRG")
            .field("prot_area_start", &self.prot_area_start())
            .field("prot_area_end", &self.prot_area_end())
            .field("dmep", &self.dmep())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - lowest PCROP protected address configuration
    #[inline(always)]
    pub fn prot_area_start(&mut self) -> PROT_AREA_START_W<'_, PRAR_PRGrs> {
        PROT_AREA_START_W::new(self, 0)
    }
    ///Bits 16:27 - highest PCROP protected address configuration
    #[inline(always)]
    pub fn prot_area_end(&mut self) -> PROT_AREA_END_W<'_, PRAR_PRGrs> {
        PROT_AREA_END_W::new(self, 16)
    }
    ///Bit 31 - PCROP protected erase enable option configuration bit
    #[inline(always)]
    pub fn dmep(&mut self) -> DMEP_W<'_, PRAR_PRGrs> {
        DMEP_W::new(self, 31)
    }
}
/**FLASH protection address for bank 1

You can [`read`](crate::Reg::read) this register and get [`prar_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prar_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#FLASH:PRAR_PRG)*/
pub struct PRAR_PRGrs;
impl crate::RegisterSpec for PRAR_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`prar_prg::R`](R) reader structure
impl crate::Readable for PRAR_PRGrs {}
///`write(|w| ..)` method takes [`prar_prg::W`](W) writer structure
impl crate::Writable for PRAR_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRAR_PRG to value 0
impl crate::Resettable for PRAR_PRGrs {}
