///Register `CSR%s` reader
pub type R = crate::R<CSRrs>;
///Register `CSR%s` writer
pub type W = crate::W<CSRrs>;
///Field `CS0` reader - Context swap x Refer to Section 34.7.7: HASH context swap registers introduction.
pub type CS0_R = crate::FieldReader<u32>;
///Field `CS0` writer - Context swap x Refer to Section 34.7.7: HASH context swap registers introduction.
pub type CS0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x Refer to Section 34.7.7: HASH context swap registers introduction.
    #[inline(always)]
    pub fn cs0(&self) -> CS0_R {
        CS0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR").field("cs0", &self.cs0()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x Refer to Section 34.7.7: HASH context swap registers introduction.
    #[inline(always)]
    pub fn cs0(&mut self) -> CS0_W<CSRrs> {
        CS0_W::new(self, 0)
    }
}
/**HASH context swap register %s

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#HASH:CSR[0])*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CSR%s to value 0
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
