///Register `HUFFBASE%s` reader
pub type R = crate::R<HUFFBASErs>;
///Register `HUFFBASE%s` writer
pub type W = crate::W<HUFFBASErs>;
///Field `HuffBase_RAM_0` reader - HuffBase RAM
pub type HUFF_BASE_RAM_0_R = crate::FieldReader<u16>;
///Field `HuffBase_RAM_0` writer - HuffBase RAM
pub type HUFF_BASE_RAM_0_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `HuffBase_RAM_1` reader - HuffBase RAM
pub type HUFF_BASE_RAM_1_R = crate::FieldReader<u16>;
///Field `HuffBase_RAM_1` writer - HuffBase RAM
pub type HUFF_BASE_RAM_1_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - HuffBase RAM
    #[inline(always)]
    pub fn huff_base_ram_0(&self) -> HUFF_BASE_RAM_0_R {
        HUFF_BASE_RAM_0_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - HuffBase RAM
    #[inline(always)]
    pub fn huff_base_ram_1(&self) -> HUFF_BASE_RAM_1_R {
        HUFF_BASE_RAM_1_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE")
            .field("huff_base_ram_0", &self.huff_base_ram_0())
            .field("huff_base_ram_1", &self.huff_base_ram_1())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - HuffBase RAM
    #[inline(always)]
    pub fn huff_base_ram_0(&mut self) -> HUFF_BASE_RAM_0_W<'_, HUFFBASErs> {
        HUFF_BASE_RAM_0_W::new(self, 0)
    }
    ///Bits 16:24 - HuffBase RAM
    #[inline(always)]
    pub fn huff_base_ram_1(&mut self) -> HUFF_BASE_RAM_1_W<'_, HUFFBASErs> {
        HUFF_BASE_RAM_1_W::new(self, 16)
    }
}
/**JPEG HuffSymb tables

You can [`read`](crate::Reg::read) this register and get [`huffbase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#JPEG:HUFFBASE[0])*/
pub struct HUFFBASErs;
impl crate::RegisterSpec for HUFFBASErs {
    type Ux = u32;
}
///`read()` method returns [`huffbase::R`](R) reader structure
impl crate::Readable for HUFFBASErs {}
///`write(|w| ..)` method takes [`huffbase::W`](W) writer structure
impl crate::Writable for HUFFBASErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE%s to value 0
impl crate::Resettable for HUFFBASErs {}
