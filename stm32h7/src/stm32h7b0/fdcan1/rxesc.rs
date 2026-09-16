///Register `RXESC` reader
pub type R = crate::R<RXESCrs>;
///Register `RXESC` writer
pub type W = crate::W<RXESCrs>;
///Field `F0DS` reader - Rx FIFO 1 Data Field Size:
pub type F0DS_R = crate::FieldReader;
///Field `F0DS` writer - Rx FIFO 1 Data Field Size:
pub type F0DS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `F1DS` reader - Rx FIFO 0 Data Field Size:
pub type F1DS_R = crate::FieldReader;
///Field `F1DS` writer - Rx FIFO 0 Data Field Size:
pub type F1DS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RBDS` reader - Rx Buffer Data Field Size:
pub type RBDS_R = crate::FieldReader;
///Field `RBDS` writer - Rx Buffer Data Field Size:
pub type RBDS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Rx FIFO 1 Data Field Size:
    #[inline(always)]
    pub fn f0ds(&self) -> F0DS_R {
        F0DS_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Rx FIFO 0 Data Field Size:
    #[inline(always)]
    pub fn f1ds(&self) -> F1DS_R {
        F1DS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - Rx Buffer Data Field Size:
    #[inline(always)]
    pub fn rbds(&self) -> RBDS_R {
        RBDS_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXESC")
            .field("f0ds", &self.f0ds())
            .field("f1ds", &self.f1ds())
            .field("rbds", &self.rbds())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Rx FIFO 1 Data Field Size:
    #[inline(always)]
    pub fn f0ds(&mut self) -> F0DS_W<'_, RXESCrs> {
        F0DS_W::new(self, 0)
    }
    ///Bits 4:6 - Rx FIFO 0 Data Field Size:
    #[inline(always)]
    pub fn f1ds(&mut self) -> F1DS_W<'_, RXESCrs> {
        F1DS_W::new(self, 4)
    }
    ///Bits 8:10 - Rx Buffer Data Field Size:
    #[inline(always)]
    pub fn rbds(&mut self) -> RBDS_W<'_, RXESCrs> {
        RBDS_W::new(self, 8)
    }
}
/**FDCAN Rx Buffer Element Size Configuration Register

You can [`read`](crate::Reg::read) this register and get [`rxesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FDCAN1:RXESC)*/
pub struct RXESCrs;
impl crate::RegisterSpec for RXESCrs {
    type Ux = u32;
}
///`read()` method returns [`rxesc::R`](R) reader structure
impl crate::Readable for RXESCrs {}
///`write(|w| ..)` method takes [`rxesc::W`](W) writer structure
impl crate::Writable for RXESCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RXESC to value 0
impl crate::Resettable for RXESCrs {}
