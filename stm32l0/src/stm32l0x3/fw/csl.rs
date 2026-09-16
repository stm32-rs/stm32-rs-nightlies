///Register `CSL` reader
pub type R = crate::R<CSLrs>;
///Register `CSL` writer
pub type W = crate::W<CSLrs>;
///Field `LENG` reader - code segment length
pub type LENG_R = crate::FieldReader<u16>;
///Field `LENG` writer - code segment length
pub type LENG_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16, crate::Safe>;
impl R {
    ///Bits 8:21 - code segment length
    #[inline(always)]
    pub fn leng(&self) -> LENG_R {
        LENG_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSL").field("leng", &self.leng()).finish()
    }
}
impl W {
    ///Bits 8:21 - code segment length
    #[inline(always)]
    pub fn leng(&mut self) -> LENG_W<'_, CSLrs> {
        LENG_W::new(self, 8)
    }
}
/**Code segment length

You can [`read`](crate::Reg::read) this register and get [`csl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#FW:CSL)*/
pub struct CSLrs;
impl crate::RegisterSpec for CSLrs {
    type Ux = u32;
}
///`read()` method returns [`csl::R`](R) reader structure
impl crate::Readable for CSLrs {}
///`write(|w| ..)` method takes [`csl::W`](W) writer structure
impl crate::Writable for CSLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSL to value 0
impl crate::Resettable for CSLrs {}
