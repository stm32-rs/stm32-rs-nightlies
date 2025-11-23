///Register `C13BR2` reader
pub type R = crate::R<C13BR2rs>;
///Register `C13BR2` writer
pub type W = crate::W<C13BR2rs>;
///Field `BRSAO` reader - Block repeated source address offset
pub type BRSAO_R = crate::FieldReader<u16>;
///Field `BRSAO` writer - Block repeated source address offset
pub type BRSAO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `BRDAO` reader - Block repeated destination address offset
pub type BRDAO_R = crate::FieldReader<u16>;
///Field `BRDAO` writer - Block repeated destination address offset
pub type BRDAO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Block repeated source address offset
    #[inline(always)]
    pub fn brsao(&self) -> BRSAO_R {
        BRSAO_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Block repeated destination address offset
    #[inline(always)]
    pub fn brdao(&self) -> BRDAO_R {
        BRDAO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C13BR2")
            .field("brsao", &self.brsao())
            .field("brdao", &self.brdao())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Block repeated source address offset
    #[inline(always)]
    pub fn brsao(&mut self) -> BRSAO_W<'_, C13BR2rs> {
        BRSAO_W::new(self, 0)
    }
    ///Bits 16:31 - Block repeated destination address offset
    #[inline(always)]
    pub fn brdao(&mut self) -> BRDAO_W<'_, C13BR2rs> {
        BRDAO_W::new(self, 16)
    }
}
/**GPDMA channel 13 block register 2

You can [`read`](crate::Reg::read) this register and get [`c13br2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13br2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPDMA:C13BR2)*/
pub struct C13BR2rs;
impl crate::RegisterSpec for C13BR2rs {
    type Ux = u32;
}
///`read()` method returns [`c13br2::R`](R) reader structure
impl crate::Readable for C13BR2rs {}
///`write(|w| ..)` method takes [`c13br2::W`](W) writer structure
impl crate::Writable for C13BR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C13BR2 to value 0
impl crate::Resettable for C13BR2rs {}
