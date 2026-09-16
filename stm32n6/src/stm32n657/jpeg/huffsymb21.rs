///Register `HUFFSYMB21` reader
pub type R = crate::R<HUFFSYMB21rs>;
///Register `HUFFSYMB21` writer
pub type W = crate::W<HUFFSYMB21rs>;
///Field `DATA84` reader - Data 84
pub type DATA84_R = crate::FieldReader;
///Field `DATA84` writer - Data 84
pub type DATA84_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA85` reader - Data 85
pub type DATA85_R = crate::FieldReader;
///Field `DATA85` writer - Data 85
pub type DATA85_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA86` reader - Data 86
pub type DATA86_R = crate::FieldReader;
///Field `DATA86` writer - Data 86
pub type DATA86_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA87` reader - Data 87
pub type DATA87_R = crate::FieldReader;
///Field `DATA87` writer - Data 87
pub type DATA87_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 84
    #[inline(always)]
    pub fn data84(&self) -> DATA84_R {
        DATA84_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 85
    #[inline(always)]
    pub fn data85(&self) -> DATA85_R {
        DATA85_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 86
    #[inline(always)]
    pub fn data86(&self) -> DATA86_R {
        DATA86_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 87
    #[inline(always)]
    pub fn data87(&self) -> DATA87_R {
        DATA87_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB21")
            .field("data84", &self.data84())
            .field("data85", &self.data85())
            .field("data86", &self.data86())
            .field("data87", &self.data87())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 84
    #[inline(always)]
    pub fn data84(&mut self) -> DATA84_W<'_, HUFFSYMB21rs> {
        DATA84_W::new(self, 0)
    }
    ///Bits 8:15 - Data 85
    #[inline(always)]
    pub fn data85(&mut self) -> DATA85_W<'_, HUFFSYMB21rs> {
        DATA85_W::new(self, 8)
    }
    ///Bits 16:23 - Data 86
    #[inline(always)]
    pub fn data86(&mut self) -> DATA86_W<'_, HUFFSYMB21rs> {
        DATA86_W::new(self, 16)
    }
    ///Bits 24:31 - Data 87
    #[inline(always)]
    pub fn data87(&mut self) -> DATA87_W<'_, HUFFSYMB21rs> {
        DATA87_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFSYMB21)*/
pub struct HUFFSYMB21rs;
impl crate::RegisterSpec for HUFFSYMB21rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb21::R`](R) reader structure
impl crate::Readable for HUFFSYMB21rs {}
///`write(|w| ..)` method takes [`huffsymb21::W`](W) writer structure
impl crate::Writable for HUFFSYMB21rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB21 to value 0
impl crate::Resettable for HUFFSYMB21rs {}
