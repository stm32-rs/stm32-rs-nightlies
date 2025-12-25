///Register `HUFFSYMB19` reader
pub type R = crate::R<HUFFSYMB19rs>;
///Register `HUFFSYMB19` writer
pub type W = crate::W<HUFFSYMB19rs>;
///Field `DATA76` reader - Data 76
pub type DATA76_R = crate::FieldReader;
///Field `DATA76` writer - Data 76
pub type DATA76_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA77` reader - Data 77
pub type DATA77_R = crate::FieldReader;
///Field `DATA77` writer - Data 77
pub type DATA77_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA78` reader - Data 78
pub type DATA78_R = crate::FieldReader;
///Field `DATA78` writer - Data 78
pub type DATA78_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA79` reader - Data 79
pub type DATA79_R = crate::FieldReader;
///Field `DATA79` writer - Data 79
pub type DATA79_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 76
    #[inline(always)]
    pub fn data76(&self) -> DATA76_R {
        DATA76_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 77
    #[inline(always)]
    pub fn data77(&self) -> DATA77_R {
        DATA77_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 78
    #[inline(always)]
    pub fn data78(&self) -> DATA78_R {
        DATA78_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 79
    #[inline(always)]
    pub fn data79(&self) -> DATA79_R {
        DATA79_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB19")
            .field("data76", &self.data76())
            .field("data77", &self.data77())
            .field("data78", &self.data78())
            .field("data79", &self.data79())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 76
    #[inline(always)]
    pub fn data76(&mut self) -> DATA76_W<'_, HUFFSYMB19rs> {
        DATA76_W::new(self, 0)
    }
    ///Bits 8:15 - Data 77
    #[inline(always)]
    pub fn data77(&mut self) -> DATA77_W<'_, HUFFSYMB19rs> {
        DATA77_W::new(self, 8)
    }
    ///Bits 16:23 - Data 78
    #[inline(always)]
    pub fn data78(&mut self) -> DATA78_W<'_, HUFFSYMB19rs> {
        DATA78_W::new(self, 16)
    }
    ///Bits 24:31 - Data 79
    #[inline(always)]
    pub fn data79(&mut self) -> DATA79_W<'_, HUFFSYMB19rs> {
        DATA79_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFSYMB19)*/
pub struct HUFFSYMB19rs;
impl crate::RegisterSpec for HUFFSYMB19rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb19::R`](R) reader structure
impl crate::Readable for HUFFSYMB19rs {}
///`write(|w| ..)` method takes [`huffsymb19::W`](W) writer structure
impl crate::Writable for HUFFSYMB19rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB19 to value 0
impl crate::Resettable for HUFFSYMB19rs {}
