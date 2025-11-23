///Register `IPGR1` reader
pub type R = crate::R<IPGR1rs>;
///Register `IPGR1` writer
pub type W = crate::W<IPGR1rs>;
///Field `MEMORYPAGE` reader - Memory page size, as power of 2 of 64-byte units:
pub type MEMORYPAGE_R = crate::FieldReader;
///Field `MEMORYPAGE` writer - Memory page size, as power of 2 of 64-byte units:
pub type MEMORYPAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `QOS_MODE` reader - Quality of service
pub type QOS_MODE_R = crate::BitReader;
///Field `QOS_MODE` writer - Quality of service
pub type QOS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Memory page size, as power of 2 of 64-byte units:
    #[inline(always)]
    pub fn memorypage(&self) -> MEMORYPAGE_R {
        MEMORYPAGE_R::new((self.bits & 7) as u8)
    }
    ///Bit 24 - Quality of service
    #[inline(always)]
    pub fn qos_mode(&self) -> QOS_MODE_R {
        QOS_MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPGR1")
            .field("memorypage", &self.memorypage())
            .field("qos_mode", &self.qos_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Memory page size, as power of 2 of 64-byte units:
    #[inline(always)]
    pub fn memorypage(&mut self) -> MEMORYPAGE_W<'_, IPGR1rs> {
        MEMORYPAGE_W::new(self, 0)
    }
    ///Bit 24 - Quality of service
    #[inline(always)]
    pub fn qos_mode(&mut self) -> QOS_MODE_W<'_, IPGR1rs> {
        QOS_MODE_W::new(self, 24)
    }
}
/**DCMIPP IPPLUG global register 1

You can [`read`](crate::Reg::read) this register and get [`ipgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:IPGR1)*/
pub struct IPGR1rs;
impl crate::RegisterSpec for IPGR1rs {
    type Ux = u32;
}
///`read()` method returns [`ipgr1::R`](R) reader structure
impl crate::Readable for IPGR1rs {}
///`write(|w| ..)` method takes [`ipgr1::W`](W) writer structure
impl crate::Writable for IPGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IPGR1 to value 0x02
impl crate::Resettable for IPGR1rs {
    const RESET_VALUE: u32 = 0x02;
}
