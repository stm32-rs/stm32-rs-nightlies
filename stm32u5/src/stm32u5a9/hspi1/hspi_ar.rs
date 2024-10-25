///Register `HSPI_AR` reader
pub type R = crate::R<HSPI_ARrs>;
///Register `HSPI_AR` writer
pub type W = crate::W<HSPI_ARrs>;
/**Field `ADDRESS` reader - Address Address to be sent to the external device. In HyperBus mode, this field must be even as this protocol is 16-bit word oriented. In dual-memory mode, AR\[0\]
is forced to 1. Writes to this field are ignored when BUSYÂ =Â 1 or when FMODE = 11 (Memory-mapped mode).*/
pub type ADDRESS_R = crate::FieldReader<u32>;
/**Field `ADDRESS` writer - Address Address to be sent to the external device. In HyperBus mode, this field must be even as this protocol is 16-bit word oriented. In dual-memory mode, AR\[0\]
is forced to 1. Writes to this field are ignored when BUSYÂ =Â 1 or when FMODE = 11 (Memory-mapped mode).*/
pub type ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    /**Bits 0:31 - Address Address to be sent to the external device. In HyperBus mode, this field must be even as this protocol is 16-bit word oriented. In dual-memory mode, AR\[0\]
    is forced to 1. Writes to this field are ignored when BUSYÂ =Â 1 or when FMODE = 11 (Memory-mapped mode).*/
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSPI_AR")
            .field("address", &self.address())
            .finish()
    }
}
impl W {
    /**Bits 0:31 - Address Address to be sent to the external device. In HyperBus mode, this field must be even as this protocol is 16-bit word oriented. In dual-memory mode, AR\[0\]
    is forced to 1. Writes to this field are ignored when BUSYÂ =Â 1 or when FMODE = 11 (Memory-mapped mode).*/
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<HSPI_ARrs> {
        ADDRESS_W::new(self, 0)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`hspi_ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspi_ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:HSPI_AR)*/
pub struct HSPI_ARrs;
impl crate::RegisterSpec for HSPI_ARrs {
    type Ux = u32;
}
///`read()` method returns [`hspi_ar::R`](R) reader structure
impl crate::Readable for HSPI_ARrs {}
///`write(|w| ..)` method takes [`hspi_ar::W`](W) writer structure
impl crate::Writable for HSPI_ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HSPI_AR to value 0
impl crate::Resettable for HSPI_ARrs {
    const RESET_VALUE: u32 = 0;
}
