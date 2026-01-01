///Register `HUFFSYMB83` reader
pub type R = crate::R<HUFFSYMB83rs>;
///Register `HUFFSYMB83` writer
pub type W = crate::W<HUFFSYMB83rs>;
///Field `DATA332` reader - Data 332
pub type DATA332_R = crate::FieldReader;
///Field `DATA332` writer - Data 332
pub type DATA332_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA333` reader - Data 333
pub type DATA333_R = crate::FieldReader;
///Field `DATA333` writer - Data 333
pub type DATA333_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA334` reader - Data 334
pub type DATA334_R = crate::FieldReader;
///Field `DATA334` writer - Data 334
pub type DATA334_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA335` reader - Data 335
pub type DATA335_R = crate::FieldReader;
///Field `DATA335` writer - Data 335
pub type DATA335_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 332
    #[inline(always)]
    pub fn data332(&self) -> DATA332_R {
        DATA332_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 333
    #[inline(always)]
    pub fn data333(&self) -> DATA333_R {
        DATA333_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 334
    #[inline(always)]
    pub fn data334(&self) -> DATA334_R {
        DATA334_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 335
    #[inline(always)]
    pub fn data335(&self) -> DATA335_R {
        DATA335_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB83")
            .field("data332", &self.data332())
            .field("data333", &self.data333())
            .field("data334", &self.data334())
            .field("data335", &self.data335())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 332
    #[inline(always)]
    pub fn data332(&mut self) -> DATA332_W<'_, HUFFSYMB83rs> {
        DATA332_W::new(self, 0)
    }
    ///Bits 8:15 - Data 333
    #[inline(always)]
    pub fn data333(&mut self) -> DATA333_W<'_, HUFFSYMB83rs> {
        DATA333_W::new(self, 8)
    }
    ///Bits 16:23 - Data 334
    #[inline(always)]
    pub fn data334(&mut self) -> DATA334_W<'_, HUFFSYMB83rs> {
        DATA334_W::new(self, 16)
    }
    ///Bits 24:31 - Data 335
    #[inline(always)]
    pub fn data335(&mut self) -> DATA335_W<'_, HUFFSYMB83rs> {
        DATA335_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb83::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb83::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB83)*/
pub struct HUFFSYMB83rs;
impl crate::RegisterSpec for HUFFSYMB83rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb83::R`](R) reader structure
impl crate::Readable for HUFFSYMB83rs {}
///`write(|w| ..)` method takes [`huffsymb83::W`](W) writer structure
impl crate::Writable for HUFFSYMB83rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB83 to value 0
impl crate::Resettable for HUFFSYMB83rs {}
