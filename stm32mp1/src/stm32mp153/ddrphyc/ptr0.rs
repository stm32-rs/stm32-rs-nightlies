///Register `PTR0` reader
pub type R = crate::R<PTR0rs>;
///Register `PTR0` writer
pub type W = crate::W<PTR0rs>;
///Field `TDLLSRST` reader - TDLLSRST
pub type TDLLSRST_R = crate::FieldReader;
///Field `TDLLSRST` writer - TDLLSRST
pub type TDLLSRST_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `TDLLLOCK` reader - TDLLLOCK
pub type TDLLLOCK_R = crate::FieldReader<u16>;
///Field `TDLLLOCK` writer - TDLLLOCK
pub type TDLLLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `TITMSRST` reader - TITMSRST
pub type TITMSRST_R = crate::FieldReader;
///Field `TITMSRST` writer - TITMSRST
pub type TITMSRST_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:5 - TDLLSRST
    #[inline(always)]
    pub fn tdllsrst(&self) -> TDLLSRST_R {
        TDLLSRST_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:17 - TDLLLOCK
    #[inline(always)]
    pub fn tdlllock(&self) -> TDLLLOCK_R {
        TDLLLOCK_R::new(((self.bits >> 6) & 0x0fff) as u16)
    }
    ///Bits 18:21 - TITMSRST
    #[inline(always)]
    pub fn titmsrst(&self) -> TITMSRST_R {
        TITMSRST_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTR0")
            .field("tdllsrst", &self.tdllsrst())
            .field("tdlllock", &self.tdlllock())
            .field("titmsrst", &self.titmsrst())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - TDLLSRST
    #[inline(always)]
    pub fn tdllsrst(&mut self) -> TDLLSRST_W<'_, PTR0rs> {
        TDLLSRST_W::new(self, 0)
    }
    ///Bits 6:17 - TDLLLOCK
    #[inline(always)]
    pub fn tdlllock(&mut self) -> TDLLLOCK_W<'_, PTR0rs> {
        TDLLLOCK_W::new(self, 6)
    }
    ///Bits 18:21 - TITMSRST
    #[inline(always)]
    pub fn titmsrst(&mut self) -> TITMSRST_W<'_, PTR0rs> {
        TITMSRST_W::new(self, 18)
    }
}
/**DDRPHYC PT register 0

You can [`read`](crate::Reg::read) this register and get [`ptr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPHYC:PTR0)*/
pub struct PTR0rs;
impl crate::RegisterSpec for PTR0rs {
    type Ux = u32;
}
///`read()` method returns [`ptr0::R`](R) reader structure
impl crate::Readable for PTR0rs {}
///`write(|w| ..)` method takes [`ptr0::W`](W) writer structure
impl crate::Writable for PTR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTR0 to value 0x0022_af9b
impl crate::Resettable for PTR0rs {
    const RESET_VALUE: u32 = 0x0022_af9b;
}
