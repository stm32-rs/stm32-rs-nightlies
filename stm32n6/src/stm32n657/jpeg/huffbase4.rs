///Register `HUFFBASE4` reader
pub type R = crate::R<HUFFBASE4rs>;
///Register `HUFFBASE4` writer
pub type W = crate::W<HUFFBASE4rs>;
///Field `DATA8` reader - Data 8
pub type DATA8_R = crate::FieldReader<u16>;
///Field `DATA8` writer - Data 8
pub type DATA8_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA9` reader - Data 9
pub type DATA9_R = crate::FieldReader<u16>;
///Field `DATA9` writer - Data 9
pub type DATA9_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 8
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 9
    #[inline(always)]
    pub fn data9(&self) -> DATA9_R {
        DATA9_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE4")
            .field("data8", &self.data8())
            .field("data9", &self.data9())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 8
    #[inline(always)]
    pub fn data8(&mut self) -> DATA8_W<'_, HUFFBASE4rs> {
        DATA8_W::new(self, 0)
    }
    ///Bits 16:24 - Data 9
    #[inline(always)]
    pub fn data9(&mut self) -> DATA9_W<'_, HUFFBASE4rs> {
        DATA9_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFBASE4)*/
pub struct HUFFBASE4rs;
impl crate::RegisterSpec for HUFFBASE4rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase4::R`](R) reader structure
impl crate::Readable for HUFFBASE4rs {}
///`write(|w| ..)` method takes [`huffbase4::W`](W) writer structure
impl crate::Writable for HUFFBASE4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE4 to value 0
impl crate::Resettable for HUFFBASE4rs {}
