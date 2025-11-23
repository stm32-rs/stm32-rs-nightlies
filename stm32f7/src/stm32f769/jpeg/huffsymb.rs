///Register `HUFFSYMB%s` reader
pub type R = crate::R<HUFFSYMBrs>;
///Register `HUFFSYMB%s` writer
pub type W = crate::W<HUFFSYMBrs>;
///Field `HuffSymb_RAM` reader - DHTSymb RAM
pub type HUFF_SYMB_RAM_R = crate::FieldReader<u32>;
///Field `HuffSymb_RAM` writer - DHTSymb RAM
pub type HUFF_SYMB_RAM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DHTSymb RAM
    #[inline(always)]
    pub fn huff_symb_ram(&self) -> HUFF_SYMB_RAM_R {
        HUFF_SYMB_RAM_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB")
            .field("huff_symb_ram", &self.huff_symb_ram())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DHTSymb RAM
    #[inline(always)]
    pub fn huff_symb_ram(&mut self) -> HUFF_SYMB_RAM_W<'_, HUFFSYMBrs> {
        HUFF_SYMB_RAM_W::new(self, 0)
    }
}
/**JPEG HUFFSYMB tables

You can [`read`](crate::Reg::read) this register and get [`huffsymb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#JPEG:HUFFSYMB[0])*/
pub struct HUFFSYMBrs;
impl crate::RegisterSpec for HUFFSYMBrs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb::R`](R) reader structure
impl crate::Readable for HUFFSYMBrs {}
///`write(|w| ..)` method takes [`huffsymb::W`](W) writer structure
impl crate::Writable for HUFFSYMBrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB%s to value 0
impl crate::Resettable for HUFFSYMBrs {}
