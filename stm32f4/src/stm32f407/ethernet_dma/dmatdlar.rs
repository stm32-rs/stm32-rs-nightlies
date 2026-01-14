///Register `DMATDLAR` reader
pub type R = crate::R<DMATDLARrs>;
///Register `DMATDLAR` writer
pub type W = crate::W<DMATDLARrs>;
///Field `STL` reader - Start of transmit list
pub type STL_R = crate::FieldReader<u32>;
///Field `STL` writer - Start of transmit list
pub type STL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Start of transmit list
    #[inline(always)]
    pub fn stl(&self) -> STL_R {
        STL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMATDLAR")
            .field("stl", &self.stl())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Start of transmit list
    #[inline(always)]
    pub fn stl(&mut self) -> STL_W<'_, DMATDLARrs> {
        STL_W::new(self, 0)
    }
}
/**Ethernet DMA transmit descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmatdlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatdlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#Ethernet_DMA:DMATDLAR)*/
pub struct DMATDLARrs;
impl crate::RegisterSpec for DMATDLARrs {
    type Ux = u32;
}
///`read()` method returns [`dmatdlar::R`](R) reader structure
impl crate::Readable for DMATDLARrs {}
///`write(|w| ..)` method takes [`dmatdlar::W`](W) writer structure
impl crate::Writable for DMATDLARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMATDLAR to value 0
impl crate::Resettable for DMATDLARrs {}
