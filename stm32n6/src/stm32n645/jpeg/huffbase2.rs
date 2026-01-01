///Register `HUFFBASE2` reader
pub type R = crate::R<HUFFBASE2rs>;
///Register `HUFFBASE2` writer
pub type W = crate::W<HUFFBASE2rs>;
///Field `DATA4` reader - Data 4
pub type DATA4_R = crate::FieldReader<u16>;
///Field `DATA4` writer - Data 4
pub type DATA4_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA5` reader - Data 5
pub type DATA5_R = crate::FieldReader<u16>;
///Field `DATA5` writer - Data 5
pub type DATA5_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 4
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 5
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE2")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 4
    #[inline(always)]
    pub fn data4(&mut self) -> DATA4_W<'_, HUFFBASE2rs> {
        DATA4_W::new(self, 0)
    }
    ///Bits 16:24 - Data 5
    #[inline(always)]
    pub fn data5(&mut self) -> DATA5_W<'_, HUFFBASE2rs> {
        DATA5_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFBASE2)*/
pub struct HUFFBASE2rs;
impl crate::RegisterSpec for HUFFBASE2rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase2::R`](R) reader structure
impl crate::Readable for HUFFBASE2rs {}
///`write(|w| ..)` method takes [`huffbase2::W`](W) writer structure
impl crate::Writable for HUFFBASE2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE2 to value 0
impl crate::Resettable for HUFFBASE2rs {}
