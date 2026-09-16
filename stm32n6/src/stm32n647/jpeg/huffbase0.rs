///Register `HUFFBASE0` reader
pub type R = crate::R<HUFFBASE0rs>;
///Register `HUFFBASE0` writer
pub type W = crate::W<HUFFBASE0rs>;
///Field `DATA0` reader - Data 0
pub type DATA0_R = crate::FieldReader<u16>;
///Field `DATA0` writer - Data 0
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA1` reader - Data 1
pub type DATA1_R = crate::FieldReader<u16>;
///Field `DATA1` writer - Data 1
pub type DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 0
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 1
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE0")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 0
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W<'_, HUFFBASE0rs> {
        DATA0_W::new(self, 0)
    }
    ///Bits 16:24 - Data 1
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W<'_, HUFFBASE0rs> {
        DATA1_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFBASE0)*/
pub struct HUFFBASE0rs;
impl crate::RegisterSpec for HUFFBASE0rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase0::R`](R) reader structure
impl crate::Readable for HUFFBASE0rs {}
///`write(|w| ..)` method takes [`huffbase0::W`](W) writer structure
impl crate::Writable for HUFFBASE0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE0 to value 0
impl crate::Resettable for HUFFBASE0rs {}
