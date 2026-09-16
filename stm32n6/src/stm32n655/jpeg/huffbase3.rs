///Register `HUFFBASE3` reader
pub type R = crate::R<HUFFBASE3rs>;
///Register `HUFFBASE3` writer
pub type W = crate::W<HUFFBASE3rs>;
///Field `DATA6` reader - Data 6
pub type DATA6_R = crate::FieldReader<u16>;
///Field `DATA6` writer - Data 6
pub type DATA6_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA7` reader - Data 7
pub type DATA7_R = crate::FieldReader<u16>;
///Field `DATA7` writer - Data 7
pub type DATA7_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 6
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 7
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE3")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 6
    #[inline(always)]
    pub fn data6(&mut self) -> DATA6_W<'_, HUFFBASE3rs> {
        DATA6_W::new(self, 0)
    }
    ///Bits 16:24 - Data 7
    #[inline(always)]
    pub fn data7(&mut self) -> DATA7_W<'_, HUFFBASE3rs> {
        DATA7_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE3)*/
pub struct HUFFBASE3rs;
impl crate::RegisterSpec for HUFFBASE3rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase3::R`](R) reader structure
impl crate::Readable for HUFFBASE3rs {}
///`write(|w| ..)` method takes [`huffbase3::W`](W) writer structure
impl crate::Writable for HUFFBASE3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE3 to value 0
impl crate::Resettable for HUFFBASE3rs {}
