///Register `HUFFSYMB30` reader
pub type R = crate::R<HUFFSYMB30rs>;
///Register `HUFFSYMB30` writer
pub type W = crate::W<HUFFSYMB30rs>;
///Field `DATA120` reader - Data 120
pub type DATA120_R = crate::FieldReader;
///Field `DATA120` writer - Data 120
pub type DATA120_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA121` reader - Data 121
pub type DATA121_R = crate::FieldReader;
///Field `DATA121` writer - Data 121
pub type DATA121_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA122` reader - Data 122
pub type DATA122_R = crate::FieldReader;
///Field `DATA122` writer - Data 122
pub type DATA122_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA123` reader - Data 123
pub type DATA123_R = crate::FieldReader;
///Field `DATA123` writer - Data 123
pub type DATA123_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 120
    #[inline(always)]
    pub fn data120(&self) -> DATA120_R {
        DATA120_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 121
    #[inline(always)]
    pub fn data121(&self) -> DATA121_R {
        DATA121_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 122
    #[inline(always)]
    pub fn data122(&self) -> DATA122_R {
        DATA122_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 123
    #[inline(always)]
    pub fn data123(&self) -> DATA123_R {
        DATA123_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB30")
            .field("data120", &self.data120())
            .field("data121", &self.data121())
            .field("data122", &self.data122())
            .field("data123", &self.data123())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 120
    #[inline(always)]
    pub fn data120(&mut self) -> DATA120_W<'_, HUFFSYMB30rs> {
        DATA120_W::new(self, 0)
    }
    ///Bits 8:15 - Data 121
    #[inline(always)]
    pub fn data121(&mut self) -> DATA121_W<'_, HUFFSYMB30rs> {
        DATA121_W::new(self, 8)
    }
    ///Bits 16:23 - Data 122
    #[inline(always)]
    pub fn data122(&mut self) -> DATA122_W<'_, HUFFSYMB30rs> {
        DATA122_W::new(self, 16)
    }
    ///Bits 24:31 - Data 123
    #[inline(always)]
    pub fn data123(&mut self) -> DATA123_W<'_, HUFFSYMB30rs> {
        DATA123_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFSYMB30)*/
pub struct HUFFSYMB30rs;
impl crate::RegisterSpec for HUFFSYMB30rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb30::R`](R) reader structure
impl crate::Readable for HUFFSYMB30rs {}
///`write(|w| ..)` method takes [`huffsymb30::W`](W) writer structure
impl crate::Writable for HUFFSYMB30rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB30 to value 0
impl crate::Resettable for HUFFSYMB30rs {}
