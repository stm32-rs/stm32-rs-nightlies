///Register `HUFFSYMB27` reader
pub type R = crate::R<HUFFSYMB27rs>;
///Register `HUFFSYMB27` writer
pub type W = crate::W<HUFFSYMB27rs>;
///Field `DATA108` reader - Data 108
pub type DATA108_R = crate::FieldReader;
///Field `DATA108` writer - Data 108
pub type DATA108_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA109` reader - Data 109
pub type DATA109_R = crate::FieldReader;
///Field `DATA109` writer - Data 109
pub type DATA109_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA110` reader - Data 110
pub type DATA110_R = crate::FieldReader;
///Field `DATA110` writer - Data 110
pub type DATA110_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA111` reader - Data 111
pub type DATA111_R = crate::FieldReader;
///Field `DATA111` writer - Data 111
pub type DATA111_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 108
    #[inline(always)]
    pub fn data108(&self) -> DATA108_R {
        DATA108_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 109
    #[inline(always)]
    pub fn data109(&self) -> DATA109_R {
        DATA109_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 110
    #[inline(always)]
    pub fn data110(&self) -> DATA110_R {
        DATA110_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 111
    #[inline(always)]
    pub fn data111(&self) -> DATA111_R {
        DATA111_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB27")
            .field("data108", &self.data108())
            .field("data109", &self.data109())
            .field("data110", &self.data110())
            .field("data111", &self.data111())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 108
    #[inline(always)]
    pub fn data108(&mut self) -> DATA108_W<'_, HUFFSYMB27rs> {
        DATA108_W::new(self, 0)
    }
    ///Bits 8:15 - Data 109
    #[inline(always)]
    pub fn data109(&mut self) -> DATA109_W<'_, HUFFSYMB27rs> {
        DATA109_W::new(self, 8)
    }
    ///Bits 16:23 - Data 110
    #[inline(always)]
    pub fn data110(&mut self) -> DATA110_W<'_, HUFFSYMB27rs> {
        DATA110_W::new(self, 16)
    }
    ///Bits 24:31 - Data 111
    #[inline(always)]
    pub fn data111(&mut self) -> DATA111_W<'_, HUFFSYMB27rs> {
        DATA111_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFSYMB27)*/
pub struct HUFFSYMB27rs;
impl crate::RegisterSpec for HUFFSYMB27rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb27::R`](R) reader structure
impl crate::Readable for HUFFSYMB27rs {}
///`write(|w| ..)` method takes [`huffsymb27::W`](W) writer structure
impl crate::Writable for HUFFSYMB27rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB27 to value 0
impl crate::Resettable for HUFFSYMB27rs {}
