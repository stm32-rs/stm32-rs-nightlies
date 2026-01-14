///Register `HUFFSYMB2` reader
pub type R = crate::R<HUFFSYMB2rs>;
///Register `HUFFSYMB2` writer
pub type W = crate::W<HUFFSYMB2rs>;
///Field `DATA8` reader - Data 8
pub type DATA8_R = crate::FieldReader;
///Field `DATA8` writer - Data 8
pub type DATA8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA9` reader - Data 9
pub type DATA9_R = crate::FieldReader;
///Field `DATA9` writer - Data 9
pub type DATA9_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA10` reader - Data 10
pub type DATA10_R = crate::FieldReader;
///Field `DATA10` writer - Data 10
pub type DATA10_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA11` reader - Data 11
pub type DATA11_R = crate::FieldReader;
///Field `DATA11` writer - Data 11
pub type DATA11_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 8
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 9
    #[inline(always)]
    pub fn data9(&self) -> DATA9_R {
        DATA9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 10
    #[inline(always)]
    pub fn data10(&self) -> DATA10_R {
        DATA10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 11
    #[inline(always)]
    pub fn data11(&self) -> DATA11_R {
        DATA11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB2")
            .field("data8", &self.data8())
            .field("data9", &self.data9())
            .field("data10", &self.data10())
            .field("data11", &self.data11())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 8
    #[inline(always)]
    pub fn data8(&mut self) -> DATA8_W<'_, HUFFSYMB2rs> {
        DATA8_W::new(self, 0)
    }
    ///Bits 8:15 - Data 9
    #[inline(always)]
    pub fn data9(&mut self) -> DATA9_W<'_, HUFFSYMB2rs> {
        DATA9_W::new(self, 8)
    }
    ///Bits 16:23 - Data 10
    #[inline(always)]
    pub fn data10(&mut self) -> DATA10_W<'_, HUFFSYMB2rs> {
        DATA10_W::new(self, 16)
    }
    ///Bits 24:31 - Data 11
    #[inline(always)]
    pub fn data11(&mut self) -> DATA11_W<'_, HUFFSYMB2rs> {
        DATA11_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFSYMB2)*/
pub struct HUFFSYMB2rs;
impl crate::RegisterSpec for HUFFSYMB2rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb2::R`](R) reader structure
impl crate::Readable for HUFFSYMB2rs {}
///`write(|w| ..)` method takes [`huffsymb2::W`](W) writer structure
impl crate::Writable for HUFFSYMB2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB2 to value 0
impl crate::Resettable for HUFFSYMB2rs {}
