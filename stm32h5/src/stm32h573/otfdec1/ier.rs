///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `SEIE` reader - Security error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when SEIF flag status is set.
pub type SEIE_R = crate::BitReader;
///Field `SEIE` writer - Security error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when SEIF flag status is set.
pub type SEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XONEIE` reader - Execute-only execute-never error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when XONEIF flag status is set.
pub type XONEIE_R = crate::BitReader;
///Field `XONEIE` writer - Execute-only execute-never error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when XONEIF flag status is set.
pub type XONEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEIE` reader - Key error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when KEIF flag status is set.
pub type KEIE_R = crate::BitReader;
///Field `KEIE` writer - Key error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when KEIF flag status is set.
pub type KEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Security error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when SEIF flag status is set.
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Execute-only execute-never error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when XONEIF flag status is set.
    #[inline(always)]
    pub fn xoneie(&self) -> XONEIE_R {
        XONEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Key error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when KEIF flag status is set.
    #[inline(always)]
    pub fn keie(&self) -> KEIE_R {
        KEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("seie", &self.seie())
            .field("xoneie", &self.xoneie())
            .field("keie", &self.keie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Security error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when SEIF flag status is set.
    #[inline(always)]
    pub fn seie(&mut self) -> SEIE_W<IERrs> {
        SEIE_W::new(self, 0)
    }
    ///Bit 1 - Execute-only execute-never error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when XONEIF flag status is set.
    #[inline(always)]
    pub fn xoneie(&mut self) -> XONEIE_W<IERrs> {
        XONEIE_W::new(self, 1)
    }
    ///Bit 2 - Key error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when KEIF flag status is set.
    #[inline(always)]
    pub fn keie(&mut self) -> KEIE_W<IERrs> {
        KEIE_W::new(self, 2)
    }
}
/**OTFDEC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#OTFDEC1:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}