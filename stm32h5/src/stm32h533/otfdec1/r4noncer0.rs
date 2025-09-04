///Register `R4NONCER0` reader
pub type R = crate::R<R4NONCER0rs>;
///Register `R4NONCER0` writer
pub type W = crate::W<R4NONCER0rs>;
///Field `REG_NONCE` reader - Region nonce, bits \[31:0\]
pub type REG_NONCE_R = crate::FieldReader<u32>;
///Field `REG_NONCE` writer - Region nonce, bits \[31:0\]
pub type REG_NONCE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Region nonce, bits \[31:0\]
    #[inline(always)]
    pub fn reg_nonce(&self) -> REG_NONCE_R {
        REG_NONCE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R4NONCER0")
            .field("reg_nonce", &self.reg_nonce())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Region nonce, bits \[31:0\]
    #[inline(always)]
    pub fn reg_nonce(&mut self) -> REG_NONCE_W<R4NONCER0rs> {
        REG_NONCE_W::new(self, 0)
    }
}
/**OTFDEC region 4 nonce register 0

You can [`read`](crate::Reg::read) this register and get [`r4noncer0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r4noncer0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#OTFDEC1:R4NONCER0)*/
pub struct R4NONCER0rs;
impl crate::RegisterSpec for R4NONCER0rs {
    type Ux = u32;
}
///`read()` method returns [`r4noncer0::R`](R) reader structure
impl crate::Readable for R4NONCER0rs {}
///`write(|w| ..)` method takes [`r4noncer0::W`](W) writer structure
impl crate::Writable for R4NONCER0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R4NONCER0 to value 0
impl crate::Resettable for R4NONCER0rs {}
