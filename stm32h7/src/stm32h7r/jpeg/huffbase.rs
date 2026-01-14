///Register `HUFFBASE%s` reader
pub type R = crate::R<HUFFBASErs>;
///Register `HUFFBASE%s` writer
pub type W = crate::W<HUFFBASErs>;
///Field `DATA0` reader - Data 0 Base Huffman value.
pub type DATA0_R = crate::FieldReader<u16>;
///Field `DATA0` writer - Data 0 Base Huffman value.
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA1` reader - Data 1 Base Huffman value.
pub type DATA1_R = crate::FieldReader<u16>;
///Field `DATA1` writer - Data 1 Base Huffman value.
pub type DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 0 Base Huffman value.
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 1 Base Huffman value.
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 0 Base Huffman value.
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W<'_, HUFFBASErs> {
        DATA0_W::new(self, 0)
    }
    ///Bits 16:24 - Data 1 Base Huffman value.
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W<'_, HUFFBASErs> {
        DATA1_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:HUFFBASE[0])*/
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
