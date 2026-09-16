///Register `HUFFBASE31` reader
pub type R = crate::R<HUFFBASE31rs>;
///Register `HUFFBASE31` writer
pub type W = crate::W<HUFFBASE31rs>;
///Field `DATA62` reader - Data 62
pub type DATA62_R = crate::FieldReader<u16>;
///Field `DATA62` writer - Data 62
pub type DATA62_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA63` reader - Data 63
pub type DATA63_R = crate::FieldReader<u16>;
///Field `DATA63` writer - Data 63
pub type DATA63_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 62
    #[inline(always)]
    pub fn data62(&self) -> DATA62_R {
        DATA62_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 63
    #[inline(always)]
    pub fn data63(&self) -> DATA63_R {
        DATA63_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE31")
            .field("data62", &self.data62())
            .field("data63", &self.data63())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 62
    #[inline(always)]
    pub fn data62(&mut self) -> DATA62_W<'_, HUFFBASE31rs> {
        DATA62_W::new(self, 0)
    }
    ///Bits 16:24 - Data 63
    #[inline(always)]
    pub fn data63(&mut self) -> DATA63_W<'_, HUFFBASE31rs> {
        DATA63_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFBASE31)*/
pub struct HUFFBASE31rs;
impl crate::RegisterSpec for HUFFBASE31rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase31::R`](R) reader structure
impl crate::Readable for HUFFBASE31rs {}
///`write(|w| ..)` method takes [`huffbase31::W`](W) writer structure
impl crate::Writable for HUFFBASE31rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE31 to value 0
impl crate::Resettable for HUFFBASE31rs {}
