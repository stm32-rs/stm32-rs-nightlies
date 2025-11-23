///Register `HUFFSYMB26` reader
pub type R = crate::R<HUFFSYMB26rs>;
///Register `HUFFSYMB26` writer
pub type W = crate::W<HUFFSYMB26rs>;
///Field `DATA104` reader - Data 104
pub type DATA104_R = crate::FieldReader;
///Field `DATA104` writer - Data 104
pub type DATA104_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA105` reader - Data 105
pub type DATA105_R = crate::FieldReader;
///Field `DATA105` writer - Data 105
pub type DATA105_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA106` reader - Data 106
pub type DATA106_R = crate::FieldReader;
///Field `DATA106` writer - Data 106
pub type DATA106_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA107` reader - Data 107
pub type DATA107_R = crate::FieldReader;
///Field `DATA107` writer - Data 107
pub type DATA107_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 104
    #[inline(always)]
    pub fn data104(&self) -> DATA104_R {
        DATA104_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 105
    #[inline(always)]
    pub fn data105(&self) -> DATA105_R {
        DATA105_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 106
    #[inline(always)]
    pub fn data106(&self) -> DATA106_R {
        DATA106_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 107
    #[inline(always)]
    pub fn data107(&self) -> DATA107_R {
        DATA107_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB26")
            .field("data104", &self.data104())
            .field("data105", &self.data105())
            .field("data106", &self.data106())
            .field("data107", &self.data107())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 104
    #[inline(always)]
    pub fn data104(&mut self) -> DATA104_W<'_, HUFFSYMB26rs> {
        DATA104_W::new(self, 0)
    }
    ///Bits 8:15 - Data 105
    #[inline(always)]
    pub fn data105(&mut self) -> DATA105_W<'_, HUFFSYMB26rs> {
        DATA105_W::new(self, 8)
    }
    ///Bits 16:23 - Data 106
    #[inline(always)]
    pub fn data106(&mut self) -> DATA106_W<'_, HUFFSYMB26rs> {
        DATA106_W::new(self, 16)
    }
    ///Bits 24:31 - Data 107
    #[inline(always)]
    pub fn data107(&mut self) -> DATA107_W<'_, HUFFSYMB26rs> {
        DATA107_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFSYMB26)*/
pub struct HUFFSYMB26rs;
impl crate::RegisterSpec for HUFFSYMB26rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb26::R`](R) reader structure
impl crate::Readable for HUFFSYMB26rs {}
///`write(|w| ..)` method takes [`huffsymb26::W`](W) writer structure
impl crate::Writable for HUFFSYMB26rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB26 to value 0
impl crate::Resettable for HUFFSYMB26rs {}
