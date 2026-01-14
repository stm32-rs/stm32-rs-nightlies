///Register `HUFFBASE1` reader
pub type R = crate::R<HUFFBASE1rs>;
///Register `HUFFBASE1` writer
pub type W = crate::W<HUFFBASE1rs>;
///Field `DATA2` reader - Data 2
pub type DATA2_R = crate::FieldReader<u16>;
///Field `DATA2` writer - Data 2
pub type DATA2_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA3` reader - Data 3
pub type DATA3_R = crate::FieldReader<u16>;
///Field `DATA3` writer - Data 3
pub type DATA3_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 2
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 3
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE1")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 2
    #[inline(always)]
    pub fn data2(&mut self) -> DATA2_W<'_, HUFFBASE1rs> {
        DATA2_W::new(self, 0)
    }
    ///Bits 16:24 - Data 3
    #[inline(always)]
    pub fn data3(&mut self) -> DATA3_W<'_, HUFFBASE1rs> {
        DATA3_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFBASE1)*/
pub struct HUFFBASE1rs;
impl crate::RegisterSpec for HUFFBASE1rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase1::R`](R) reader structure
impl crate::Readable for HUFFBASE1rs {}
///`write(|w| ..)` method takes [`huffbase1::W`](W) writer structure
impl crate::Writable for HUFFBASE1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE1 to value 0
impl crate::Resettable for HUFFBASE1rs {}
